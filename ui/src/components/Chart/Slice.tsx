import React, { useMemo } from 'react';
import Color from 'color';
import * as d3 from 'd3'; // @to-do should change the package to just d3 modules we are actually using

const mutateSlice = (color, highlight) => (target) => {
  const fill = highlight ? Color(color).lighten(0.05) : color;
  d3.select(event.target.parentNode)
    .select('path')
    .style('fill', fill)
    .attr('transform', (d, i) => '');
};

const Slice = (props) => {
  let { pie, size } = props;

  const radius = Math.min(size[0], size[1]) / 2;
  const arc = d3.arc().innerRadius(0).outerRadius(radius);

  const color = d3.scaleOrdinal(d3.schemePaired);

  return pie.map((slice, index) => {
    const sliceColor = color(slice.data.key);

    const onMouseOver = useMemo(() => mutateSlice(sliceColor, true), [
      sliceColor,
      true,
    ]);

    const onMouseOut = mutateSlice(sliceColor, false);

    const [xPos, yPos] = arc.centroid(slice);

    const midangle = slice.startAngle + (slice.endAngle - slice.startAngle) / 2;

    return (
      <g key={slice.data.key} onMouseOver={onMouseOver} onMouseOut={onMouseOut}>
        <path d={arc(slice)} fill={sliceColor} />
        <text x={xPos} y={yPos} fill="#FFFFFF" style={{ fontSize: '0.5em' }}>
          {slice.data.key}
        </text>
      </g>
    );
  });
};

export default Slice;
