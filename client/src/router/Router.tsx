import React from 'react';
import { Route, Switch } from 'react-router-dom';

import Home from '../pages/Home/Home';
import Lobby from '../pages/Lobby/Lobby';
import * as routes from './routes';

const Router = () => (
  <Switch>
    <Route exact path={routes.HOME} component={Home} />
    <Route path={routes.LOBBY} component={Lobby} />
  </Switch>
);

export default Router;
