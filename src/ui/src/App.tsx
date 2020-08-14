import React, { useEffect, useState } from 'react';
import './App.css';
import axios from 'axios';

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
      <div className="App-container">Here</div>
    </div>
  );
};

export default App;
