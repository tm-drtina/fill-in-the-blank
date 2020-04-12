import React from 'react';
import { Route, Switch } from 'react-router-dom';

import Home from '../pages/Home/Home';
import * as routes from './routes';

const Router = () => (
  <Switch>
    <Route path={routes.HOME} component={Home} />
  </Switch>
);

export default Router;
