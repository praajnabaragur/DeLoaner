import './App.css'

import Payment from './components/Payment'
import { ConnectWallet } from './components/ConnectWallet'
import WalletPage from './components/WalletPage'
import WalletTransfer from './components/WalletTransfer'
import Smallbiz from './smallbiz'
import Home from './components/Home'

import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import TextField from "@mui/material/TextField";
import Stack from "@mui/material/Stack";
import Grid from "@mui/material/Grid";

import { useEffect, useState } from 'react'
import {
  useWallet,
  useConnectedWallet,
  WalletStatus,
} from '@terra-money/wallet-provider'

import * as execute from './contract/execute'
import * as query from './contract/query'
// import { ConnectWallet } from './components/ConnectWallet'

const URL_TMP = "https://api.qrserver.com/v1/create-qr-code/?size=150x150&data=%s"

function App() {
  const [count, setCount] = useState(null)
  const [updating, setUpdating] = useState(true)
  const [resetValue, setResetValue] = useState(0)
  const [url, setUrl] = useState('')
  const [qrUrl, setQrUrl] = useState('')
  const [isQrUrlSubmitted, setIsQrUrlSubmitted] = useState(false)
  const [pageNum, setPageNum] = useState(7)

  const { status } = useWallet()

  const connectedWallet = useConnectedWallet()

  useEffect(() => {
    const prefetch = async () => {
      if (connectedWallet) {
        setCount((await query.getCount(connectedWallet)).count)
      }
      setUpdating(false)
    }
    prefetch()
  }, [connectedWallet])

  const onClickIncrement = async () => {
    setUpdating(true)
    await execute.increment(connectedWallet)
    setCount((await query.getCount(connectedWallet)).count)
    setUpdating(false)
  }

  const onClickReset = async () => {
    setUpdating(true)
    console.log(resetValue)
    await execute.reset(connectedWallet, resetValue)
    setCount((await query.getCount(connectedWallet)).count)
    setUpdating(false)
  }

  const handleSubmit = (e) => {
    e.preventDefault()
    let user_url = URL_TMP.replace("%s", url)
    setIsQrUrlSubmitted(true)
    setQrUrl(user_url)
  }

  // Example:
  {/* <header className="App-header">
        <div style={{ display: 'inline' }}>
          COUNT: {count} {updating ? '(updating . . .)' : ''}
          <button onClick={onClickIncrement} type="button">
            {' '}
            +{' '}
          </button>
        </div>
        {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <input
              type="number"
              onChange={(e) => setResetValue(+e.target.value)}
              value={resetValue}
            />
            <button onClick={onClickReset} type="button">
              {' '}s
              reset{' '}
            </button>
          </div>
        )}
        <ConnectWallet />
      </header> */}

  return (
    <div className="App">
      <Box >
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h4" component="div" >
            Deloaner
          </Typography> 
          <Box sx={{ flexGrow: 1 }} />
          <Button onClick={() => setPageNum(7)} color="inherit">Home</Button>
          <Button onClick={() => setPageNum(1)} color="inherit">Auth</Button>
          <Button onClick={() => setPageNum(2)} color="inherit">Dashboard</Button>
          <Button onClick={() => setPageNum(3)} color="inherit">Payment</Button>
          <Button onClick={() => setPageNum(4)} color="inherit">Connect</Button>
          <Button onClick={() => setPageNum(5)} color="inherit">Login</Button>
          <Button onClick={() => setPageNum(6)} color="inherit">Transfer</Button>
        </Toolbar>
      </AppBar>
    </Box>  
      {pageNum === 7 && <Home />}
      {pageNum === 1 && (
        <form onSubmit={handleSubmit}>
          <div style={{marginTop: '25px',marginLeft:'600px'}}>
          <Box sx={{ width: "100%", maxWidth: 200, bgcolor: "background.paper" }}>
          <Grid item xs={12} md={6}>
          <Stack direction="column" spacing={2}>
          <TextField
            required
            id="Website URL"
            label="Website URL"
            defaultValue=""
            value={url}
            onChange={(e) => setUrl(e.target.value)}
          />
        
        <button type='submit'> Creat QR Code</button>
        <button onClick={() => setPageNum(2)}> Business Owner Dashboard</button>
        <button onClick={() => window.open(qrUrl)}>{isQrUrlSubmitted ? "View QR Code" : "Not Submitted"}</button>
        </Stack>
        </Grid>
        </Box>
        </div>
      </form>
      )}
      {pageNum === 2 && (
        <Smallbiz />
      )}
      {pageNum === 3 && (
          <Payment pageNum={pageNum} setPageNum={setPageNum} />
      )}
      {pageNum === 4 && (
       <ConnectWallet />
      )}
      {pageNum === 5 && (
      <WalletPage pageNum={pageNum} setPageNum={setPageNum} />
      
      )}
      {pageNum === 6 && (
      <WalletTransfer  />
      )}
    </div>
  )
}

export default App
