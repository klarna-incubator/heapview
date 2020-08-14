import React from 'react';
import './App.css';
import Chart from './components/Chart';

const exampleContract = {
  totalBytes: 100,
  categories: [
    ['a', 30],
    ['b', 30],
    ['c', 40],
  ],
};

const d3exampleData = [30, 30];

interface AppProps {}

function App({}: AppProps) {
  return (
    <div className="App">
      <Chart data={d3exampleData} />
    </div>
  );
}

export default App;
