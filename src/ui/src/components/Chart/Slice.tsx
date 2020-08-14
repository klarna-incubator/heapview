import React from 'react';
import * as d3 from 'd3'; // @to-do should change the package to just d3 modules we are actually using

const Slice = (props) => {
  let { pie } = props;

  let arc = d3.arc().innerRadius(0).outerRadius(100);

  let interpolate = d3.interpolateRgb('#eaaf79', '#bc3358');

  return pie.map((slice, index) => {
    let sliceColor = interpolate(index / (pie.length - 1));

    return <path d={arc(slice)} fill={sliceColor} />;
  });
};

export default Slice;
