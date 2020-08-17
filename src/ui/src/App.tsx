import React, { useEffect, useState } from 'react';
import './App.css';
import axios from 'axios';
import Chart from './components/Chart';

interface AppProps {}

const analysisUrl =
  process.env.NODE_ENV === 'development'
    ? 'http://localhost:3000/analysis'
    : '/analysis';

const App = ({}: AppProps) => {
  const [data, setData] = useState(false);
  useEffect(() => {
    const requestFromServer = async () => {
      const res = await axios.request({
        url: analysisUrl,
      });
      setData(res.data);
    };
    requestFromServer();
  }, []);

  return (
    <div className="App">
      <h1>
        <a
          href={'https://github.com/klarna-incubator/heapview'}
          target={'_blank'}
        >
          heapview
        </a>
      </h1>
      {data && <Chart data={data} />}
    </div>
  );
};

export default App;
