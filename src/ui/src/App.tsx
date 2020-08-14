import React, { useEffect, useState } from 'react';
import './App.css';
import axios from 'axios';
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

const App = ({}: AppProps) => {
  const [data, setData] = useState({});
  useEffect(() => {
    const requestFromServer = async () => {
      const res = await axios.request({
        url: 'https://reqres.in/api/unknown',
      });
      setData(res.data);
    };
    requestFromServer();
  }, []);

  console.log(data);
  return (
    <div className="App">
      <div className="App-container">
        <Chart data={d3exampleData} />
      </div>
    </div>
  );
};

export default App;
