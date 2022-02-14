import { Route, Routes } from 'react-router-dom';

import Home from '../pages/Home/Home';
import Lobby from '../pages/Lobby/Lobby';

const Router = () => (
  <Routes>
    <Route path='/' element={<Home/>} />
    <Route path='/lobby/:id' element={<Lobby/>}/>
  </Routes>
);

export default Router;
