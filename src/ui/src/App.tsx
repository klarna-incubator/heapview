import React, { useEffect, createRef } from 'react';
import * as d3 from 'd3'; // @to-do should change the package to just d3 modules we are actually using
import logo from './logo.svg';
import './App.css';

const exampleContract = {
  totalBytes: 100,
  categories: [
    ['a', 30],
    ['b', 30],
    ['c', 40],
  ],
};

const d3exampleData = [30, 30, 40];

interface AppProps {}

function App({}: AppProps) {
  const sunburstChartRef = createRef();

  /** @to-do Should this be configurable by user? */
  const width = 500;
  const height = 500;

  /** @to-do switch to exampleContract and add labels */
  let pie = d3.pie()(d3exampleData);

  useEffect(() => {});

  return (
    <div className="App">
      <svg height={height} width={width} ref={sunburstChartRef}>
        <g transform={`translate(${width / 2},${height / 2})`}>
          <Slice pie={pie} />
        </g>
      </svg>
    </div>
  );
}

const Slice = (props) => {
  let { pie } = props;

  let arc = d3.arc().innerRadius(0).outerRadius(100);

  let interpolate = d3.interpolateRgb('#eaaf79', '#bc3358');

  return pie.map((slice, index) => {
    let sliceColor = interpolate(index / (pie.length - 1));

    return <path d={arc(slice)} fill={sliceColor} />;
  });
};

export default App;
