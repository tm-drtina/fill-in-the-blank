import React from 'react';
import Modal from '@mui/material/Modal';
import { Theme } from '@mui/material';

import makeStyles from '@mui/styles/makeStyles';
import createStyles from '@mui/styles/createStyles';

interface IProps {
  open: boolean;
  onClose: () => void;
  children: React.ReactNode;
}

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    modalBody: {
      position: 'absolute',
      top: '50%',
      left: '50%',
      transform: 'translate(-50%, -50%)',
      width: 400,
      backgroundColor: theme.palette.background.paper,
      boxShadow: theme.shadows[5],
      padding: theme.spacing(2, 4, 3),
      '&:focus': {
        outline: 'none',
      },
    },
  })
);

const Dialog = ({ open, onClose, children }: IProps) => {
  const { modalBody } = useStyles();
  return (
    <Modal open={open} onClose={onClose}>
      <div className={modalBody}>{children}</div>
    </Modal>
  );
};

export default Dialog;
