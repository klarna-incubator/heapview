import React, { useMemo, SyntheticEvent } from 'react';
import Color from 'color';
import { select as d3select } from 'd3-selection';
import { arc as d3arc } from 'd3-shape';

interface SliceProps {
  pie: Object;
  width: number;
  height: number;
}

const mutateSlice = (color: string, highlight: boolean) => (
  event: SyntheticEvent,
) => {
  const fill = highlight ? Color(color).lighten(0.1) : color;
  d3select(event.target.parentNode).select('path').style('fill', fill);
};

const Slice = ({ pie, width, height }: SliceProps) => {
  const radius = Math.min(width, height) / 2;
  const arc = d3arc()
    .innerRadius(radius / 2)
    .outerRadius(radius);

  return pie.map((slice) => {
    const sliceColor = slice.data.color;

    const onMouseOver = useMemo(() => mutateSlice(sliceColor, true), [
      sliceColor,
      true,
    ]);

    const onMouseOut = mutateSlice(sliceColor, false);

    return (
      <g key={slice.data.key} onMouseOver={onMouseOver} onMouseOut={onMouseOut}>
        <path d={arc(slice)} fill={sliceColor} />
        <title>{slice.data.key}</title>
      </g>
    );
  });
};

export default Slice;
