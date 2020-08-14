import React, { useEffect, createRef } from 'react';
import Slice from './Slice';
import * as d3 from 'd3'; // @to-do should change the package to just d3 modules we are actually using
import { map, applySpec, head, last } from 'ramda';

interface Data {
  categories: Array<any>;
  totalBytes: number;
}
interface ChartProps {
  data: Data;
}

const applyKeyValue = applySpec({
  key: head,
  value: last,
});

const transformEntries = map(applyKeyValue);

const Chart = ({ data }: ChartProps) => {
  const { categories } = data;
  const sunburstChartRef = createRef();

  useEffect(() => {
    /** @to-do use d3 append / selectors */
    if (categories && sunburstChartRef.current) {
      const svg = d3.select(sunburstChartRef.current);
    }
  });

  /** @to-do Should this be configurable by user? */
  const width = 500;
  const height = 500;

  const entries = transformEntries(categories);

  /** @to-do switch to exampleContract and add labels */
  let pie = d3
    .pie()
    .sort(null)
    .value((d) => d.value)(entries);

  return (
    <svg height={height} width={width} ref={sunburstChartRef}>
      <g transform={`translate(${width / 2},${height / 2})`}>
        <Slice pie={pie} size={[width, height]} />
      </g>
    </svg>
  );
};

export default Chart;
