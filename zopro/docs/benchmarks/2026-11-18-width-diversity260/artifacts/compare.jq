def median7: sort | if length == 7 then .[3] else error("expected seven blocks") end;
def arms: ["accepted","parent","neither","width","cert","both","both32","aeglos"];
def ratio($block;$pair;$field):
  ($block[$pair[0]] | getpath($field)) / ($block[$pair[1]] | getpath($field));
if (.passed and .unique == 65133 and (.rows|length) == 64) then . else error("incomplete capture") end
| .rows as $rows
| ([$rows[]|select(.job.pre==false)] | sort_by(.job.block) | group_by(.job.block) |
   map(if (map(.job.arm)|sort) != (arms|sort) then error("incomplete block") else INDEX(.job.arm) end)) as $blocks
| [["cert","neither"],["both","width"],["width","neither"],["both","cert"],["both","accepted"],
   ["both","aeglos"],["both","both32"],["neither","accepted"],["width","parent"]] as $pairs
| {
  arms: [arms[] as $arm | {
    arm:$arm, ms:([$blocks[] | .[$arm].means.full]|median7), cpu_ms:([$blocks[]|.[$arm].cpu_ms]|median7),
    peak_mib:([$rows[]|select(.job.arm==$arm)|.rss_mib]|max)
  }],
  paired: [$pairs[] as $p | ([$blocks[]|ratio(.;$p;["means","full"])]) as $wall |
    {pair:($p|join("/")),block_ratios:$wall,median:($wall|median7),wins:([$wall[]|select(.<1)]|length),
     cpu_median:([$blocks[]|ratio(.;$p;["cpu_ms"])]|median7)}],
  interaction: ([$blocks[]|.both.means.full*.neither.means.full/(.cert.means.full*.width.means.full)] |
    {blocks:.,median:median7}),
  aeglos_stage_ms: (reduce ($blocks[0].aeglos.means | keys[]) as $k ({}; .[$k]=([$blocks[]|.aeglos.means[$k]]|median7)))
}
