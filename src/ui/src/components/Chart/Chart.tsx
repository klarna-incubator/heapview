import React, { useEffect, createRef } from 'react';
import Slice from './Slice';
import * as d3 from 'd3'; // @to-do should change the package to just d3 modules we are actually using

interface ChartProps {
  data: Array<number>;
}

const Chart = ({ data }: ChartProps) => {
  const sunburstChartRef = createRef();

  /** @to-do Should this be configurable by user? */
  const width = 500;
  const height = 500;

  /** @to-do switch to exampleContract and add labels */
  let pie = d3.pie()(data);

  return (
    <svg height={height} width={width} ref={sunburstChartRef}>
      <g transform={`translate(${width / 2},${height / 2})`}>
        <Slice pie={pie} />
      </g>
    </svg>
  );
};

export default Chart;
