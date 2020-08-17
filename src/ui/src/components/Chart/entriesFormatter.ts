import { applySpec, head, last, compose, map, toPairs } from 'ramda';
import { randomColor } from '../../utils';

const formatToD3 = applySpec({
  key: head,
  value: last,
  color: compose(randomColor, last),
});

const entriesFormatter = compose(map(formatToD3), toPairs);

export default entriesFormatter;
