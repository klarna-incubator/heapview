import React from 'react';
import Slice from './Slice';
import ReferenceTable from '../ReferenceTable';
import entriesFormatter from './entriesFormatter';
import { pie as d3pie } from 'd3-shape';
import './Chart.css';

interface Data {
  categories: Array<any>;
  total: number;
}
interface ChartProps {
  data: Data;
}

const Chart = ({ data }: ChartProps) => {
  const { categories, total } = data;

  /** @to-do Should this be configurable by user? */
  const width = 500;
  const height = 500;

  const entries = entriesFormatter(categories);

  const pie = d3pie().value((d) => d.value)(entries);

  return (
    <div className="chart container">
      <div>
        <svg height={height} width={width}>
          <g transform={`translate(${width / 2},${height / 2})`}>
            <Slice pie={pie} height={height} width={width} />
          </g>
        </svg>
      </div>
      <div>
        <ReferenceTable pie={pie} total={total} />
      </div>
    </div>
  );
};

export default Chart;
