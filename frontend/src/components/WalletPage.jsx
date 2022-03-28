import * as React from 'react';
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import Grid from '@mui/material/Grid';
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import WalletTransfer from './WalletTransfer';

export default function FormPropsTextFields({ pageNum, setPageNum }) {

  const handleSubmit = (e) => {
    e.preventDefault()
  }
  return (
    <div style={{marginTop: '25px',marginLeft:'600px'}}>
    <Box sx={{ width: '100%', maxWidth: 200, bgcolor: 'background.paper'}}>

    <Grid item xs={12} md={6}>
          <Typography sx={{ mt: 4, mb: 2 }} variant="h6" component="div">
            Have a wallet?
          </Typography>

      <Stack direction="column" spacing={2}>
        <TextField
          required
          id="user name"
          label="user name"
          defaultValue=""
        />
        <TextField
          required
          id="outlined-password-input"
          label="Password"
          type="password"
          autoComplete="current-password"
        />
        <form onSubmit={handleSubmit}></form>
        <Button variant="outlined" onClick={() => setPageNum(6)}>Sign In</Button>
      </Stack> 
    </Grid>
    <Grid item xs={12} md={6}>
      <Typography sx={{ mt: 4, mb: 2 }} variant="h6" component="div">
            Don't have a wallet? 
      </Typography>
      
      <Button variant="outlined" onClick={() => setPageNum(6)}>Make one </Button>
    </Grid>

    </Box>
    </div>
  );
}