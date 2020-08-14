import React from 'react';
import * as d3 from 'd3'; // @to-do should change the package to just d3 modules we are actually using

const Slice = (props) => {
  let { pie, size } = props;

  const radius = Math.min(size[0], size[1]) / 2;
  const arc = d3.arc().innerRadius(0).outerRadius(radius);

  const interpolate = d3.interpolateRgb('#FFB3C7', '#F1DED0');

  return pie.map((slice, index) => {
    const sliceColor = interpolate(index / (pie.length - 1));

    const textPos = arc.centroid(slice);
    const midangle = slice.startAngle + (slice.endAngle - slice.startAngle) / 2;

    return (
      <>
        <path d={arc(slice)} fill={sliceColor} />
        <text x={textPos[0]} y={textPos[1]} fill="#FFFFFF">
          {slice.data.key}
        </text>
      </>
    );
  });
};

export default Slice;
