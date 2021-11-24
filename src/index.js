import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter, Routes, Route, } from 'react-router-dom';
import './index.css';
import SplashScreen from './Screens/SplashScreen';
import MainScreen from './Screens/MainScreen';
import InstallScreen from './Screens/InstallScreen';

ReactDOM.render(
    <React.StrictMode>
        <BrowserRouter>
            <Routes>
                <Route exact path="/:pageName" element={MainScreen} />
                <Route path="/splashscreen" element={<SplashScreen enableTips={true} />} />
                <Route path="/install" element={<InstallScreen />} />
            </Routes>
        </BrowserRouter>
    </React.StrictMode>,
    document.getElementById('root')
);
