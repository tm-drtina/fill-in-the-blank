import { ThemeProvider } from '@material-ui/core/styles';
import { StoreProvider } from 'easy-peasy';
import { SnackbarProvider } from 'notistack';
import React from 'react';
import { HashRouter } from 'react-router-dom';
import WebSocketWrapper from './components/WebSocketWrapper/WebSocketWrapper';

import store from './store';
import theme from './theme';
import { CssBaseline } from "@material-ui/core";

function App() {
    return (
        <StoreProvider store={store}>
            <CssBaseline/>
            <HashRouter>
                <ThemeProvider theme={theme}>
                    <SnackbarProvider maxSnack={3}>
                        <WebSocketWrapper/>
                    </SnackbarProvider>
                </ThemeProvider>
            </HashRouter>
        </StoreProvider>
    );
}

export default App;
