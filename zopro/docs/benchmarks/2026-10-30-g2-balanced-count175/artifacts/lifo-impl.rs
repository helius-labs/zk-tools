impl Model {
fn take_from_queue(&mut self) {
        while let Some(&(encoded, p)) = self.queue.last() {
            if self.busy((encoded & 0x7fffffff) as usize) {
                return;
            }
            self.queue.pop();
            self.add(encoded, p);
        }
    }
}
