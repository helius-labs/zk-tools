export function threadPool(spawn) {
  const workers = new Set();
  const pending = new Set();

  async function close() {
    for (const finish of pending)
      finish(new Error("Rayon pool closed during initialization"));
    const active = [...workers];
    workers.clear();
    await Promise.all(active.map((worker) => worker.terminate()));
  }

  async function startWorkers(module, memory, builder) {
    const count = builder.numThreads();
    if (!Number.isInteger(count) || count < 1 || count > 64)
      throw new RangeError("Invalid Rayon pool size");
    if (workers.size) throw new Error("The Rayon pool has already started");
    try {
      await Promise.all(
        Array.from({ length: count }, async () => {
          const worker = spawn();
          workers.add(worker);
          return new Promise((resolve, reject) => {
            let settled = false;
            const timeout = setTimeout(
              () => finish(new Error("Rayon worker initialization timed out")),
              30_000,
            );
            const unlisten = worker.listen(
              (message) => {
                if (message?.type === "ready") finish();
              },
              (error) => finish(error),
            );
            function finish(error) {
              if (settled) return;
              settled = true;
              clearTimeout(timeout);
              unlisten();
              pending.delete(finish);
              if (error) reject(error);
              else resolve();
            }
            pending.add(finish);
            try {
              worker.send({ module, memory, receiver: builder.receiver() });
            } catch (error) {
              finish(error);
            }
          });
        }),
      );
      builder.build();
    } catch (error) {
      await close();
      throw error;
    }
  }

  return { startWorkers, close };
}
