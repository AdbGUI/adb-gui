import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter, Routes, Route, } from 'react-router-dom';
import './index.css';
import SplashScreen from './Screens/SplashScreen';
import MainScreen from './Screens/MainScreen';

ReactDOM.render(
    <React.StrictMode>
        <BrowserRouter>
            <Routes>
                <Route path="/splashscreen" element={<SplashScreen enableTips={true} />} />
                <Route exact path="/" element={<MainScreen />} />
            </Routes>
        </BrowserRouter>
    </React.StrictMode>,
    document.getElementById('root')
);
