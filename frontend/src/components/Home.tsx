import React from 'react'
import Typography from '@mui/material/Typography'
import Button from '@mui/material/Button'
import Box from '@mui/material/Box'

// Note (order):
// front -> left -> back -> right

const home = () => {
  return (
    <div>
      {/* Full Screen */}
      <Box /*sx={{ display: { xs: 'none', lg: 'block' } }}*/>
        <Box
          className='backgroundgradient'
          style={{ width: '100%', height: '50rem' }}
        >
          <Box
            sx={{
              position: 'absolute',
              marginTop: '17.5rem',
              marginLeft: { xs: 0, sm: '2rem', md: '5rem' },
            }}
          >
              <Typography className='font-chakra' sx={{ fontSize: '4rem' }}>
                Deloaner
              </Typography>
            <Typography
              variant='h6'
              className='font-chakra'
              sx={{ marginBottom: '2.5rem' }}
            >
              A loan provider for small businesses
            </Typography>
                <Button
                  variant='contained'
                  className='btn'
                  sx={{ width: '7.5rem', marginRight: '1rem' }}
                >
                  Auth
                </Button>
                <Button
                  variant='outlined'
                  className='btn'
                  sx={{ width: '7.5rem' }}
                >
                  Dashboard
                </Button>
          </Box>
          <Box sx={{ display: { xs: 'block', lg: 'none' } }}>
            <div className='cube' />
          </Box>
        </Box>
      </Box>
    </div>
  )
}

export default home