import { styled } from '@mui/material';
import React from 'react';

import Loader from './Loader';

const GlobalWrapper = styled('div')(() => ({
  position: 'fixed',
  top: 0,
  left: 0,
  width: '100%',
  zIndex: 9999,
}));

type Props = {
  loaders: number;
};

const LoaderGlobal: React.FC<Props> = ({ loaders }) => (
  <>
    {loaders > 0 && (
      <GlobalWrapper>
        <Loader />
      </GlobalWrapper>
    )}
  </>
);

export default LoaderGlobal;
