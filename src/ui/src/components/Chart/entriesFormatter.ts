import {
  applySpec,
  head,
  last,
  compose,
  map,
  toPairs,
  sortWith,
  descend,
  prop,
} from 'ramda';
import { randomColor } from '../../utils';

const formatToD3 = applySpec({
  key: head,
  value: last,
  color: compose(randomColor, last),
});

const entriesFormatter = compose(
  sortWith([descend(prop('value'))]),
  map(formatToD3),
  toPairs,
);

export default entriesFormatter;
