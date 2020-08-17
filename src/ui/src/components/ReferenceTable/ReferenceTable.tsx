import React from 'react';
import './ReferenceTable.css';

const formatNumber = (number) =>
  `${new Intl.NumberFormat().format(Math.round(number / 1000))} KB`;

const ColoredSquare = ({ color }) => (
  <div className="colored-square" style={{ backgroundColor: color }} />
);

const ReferenceTable = ({ pie, total }) => {
  return (
    <table className="reference">
      <tbody>
        {pie.map(({ data }) => (
          <tr key={data.key}>
            <td className="size">{formatNumber(data.value)}</td>
            <td>
              <ColoredSquare color={data.color} />
            </td>
            <td>{data.key}</td>
          </tr>
        ))}
        <tr className="total">
          <td>{formatNumber(total)}</td>
          <td></td>
          <td>Total</td>
        </tr>
      </tbody>
    </table>
  );
};

export default ReferenceTable;
