import { ThemeProvider } from '@material-ui/core/styles';
import { StoreProvider } from 'easy-peasy';
import React from 'react';
import { HashRouter } from 'react-router-dom';
import WebSocketWrapper from './components/WebSocketWrapper/WebSocketWrapper';

import store from './store';
import theme from './theme';

function App() {
  return (
    <StoreProvider store={store}>
      <HashRouter>
        <ThemeProvider theme={theme}>
          <WebSocketWrapper/>
        </ThemeProvider>
      </HashRouter>
    </StoreProvider>
  );
}

export default App;
