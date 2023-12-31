import UpSetJS, { extractCombinations } from '@upsetjs/react';
import type { QueriedRecordResult } from '@types';
import { useMemo } from 'react';
// import the upsetjs react component

function QueryResultGraph({ queryResult }: { queryResult: QueriedRecordResult[] }) {

  // const elems = [
  //   { name: 'A', sets: ['S1', 'S2'] },
  //   { name: 'B', sets: ['S1'] },
  //   { name: 'C', sets: ['S2'] },
  //   { name: 'D', sets: ['S1', 'S3'] },
  // ];
  const elements = queryResult.map((result) => {
    return {
      name: result.id,
      sets: result.tags.concat(result.mimetype),
    };
  });

  console.log({elements})

  const { sets, combinations } = useMemo(() => extractCombinations(elements), [queryResult]);
  const UpSet = UpSetJS as any;

  return (
    <section id="query-result-graph">
      {/* <UpSet sets={sets} combinations={combinations} width={500} height={300} /> */}
    </section>
  );
}

export default QueryResultGraph;
