import React, { useEffect, useState } from 'react';
import './App.css';
import axios from 'axios';
import Chart from './components/Chart';

const exampleContract = {
  totalBytes: 100,
  categories: [
    ['a', 20],
    ['b', 40],
    ['c', 40],
  ],
};

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
        <Chart data={exampleContract} />
      </div>
    </div>
  );
};

export default App;
