import { ThemeProvider, StyledEngineProvider } from '@mui/material/styles';
import { StoreProvider } from 'easy-peasy';
import { SnackbarProvider } from 'notistack';
import { HashRouter } from 'react-router-dom';
import WebSocketWrapper from './components/WebSocketWrapper/WebSocketWrapper';

import store from './store';
import theme from './theme';

function App() {
  return (
    <StoreProvider store={store}>
      <HashRouter>
        <StyledEngineProvider injectFirst>
          <ThemeProvider theme={theme}>
            <SnackbarProvider maxSnack={3}>
              <WebSocketWrapper/>
            </SnackbarProvider>
          </ThemeProvider>
        </StyledEngineProvider>
      </HashRouter>
    </StoreProvider>
  );
}

export default App;
