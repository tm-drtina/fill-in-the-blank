import React from 'react';

import Router from '../../router/Router';
import Footer from '../Footer/Footer';
import Navbar from '../Navbar/Navbar';

const Layout: React.FC = () => (
  <>
    <Navbar/>
    <Router/>
    <Footer/>
  </>
);

export default Layout;
