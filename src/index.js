import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import SplashScreen from './Screens/SplashScreen';

ReactDOM.render(
    <React.StrictMode>
        <SplashScreen enableTips={true} />
    </React.StrictMode>,
    document.getElementById('root')
);
