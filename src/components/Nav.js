import { Anchor, Box, Button, Header, Layer, ResponsiveContext, Text, Select } from 'grommet';
import { Apps, Close } from 'grommet-icons';
import React, { useContext } from 'react';
import { useSnapshot } from 'valtio';
import { appState } from '../App';
import { History } from './History';
import { utils } from 'near-api-js';


const ConnectBox = ({ size, accountId, wallet, isLogged, explorerUrl }) => {
  return <Box direction={size === 'small' ? 'column' : 'row'} align='center' gap='small'>
    {isLogged && <Anchor target='_blank' href={`${explorerUrl}/accounts/${accountId}`} label={accountId} size='small' />}
    {isLogged ?
      (
        <Button label={'Disconnect'} size='small' onClick={() => {
          localStorage.removeItem('gawibawibo_wallet_auth_key');
          window.location.reload()
        }} />
      ) :
      (
        <Button label={'Connect'} size='small' onClick={() => {
          wallet.requestSignIn({ successUrl: 'http://localhost:3000' })
        }} />
      )
    }
  </Box>
}

const ResponsiveMenu = ({ toggleHistory, size, accountId, wallet, isLogged, explorerUrl, unclaimedAmount, handleClaim, themeMode }) => {
  const [openMenu, setOpenMenu] = React.useState(false);
  const toggleMenu = () => setOpenMenu((value) => !value);

  return <Box align='center' justify='end'>
    <Button label={<Apps />} size='medium' onClick={toggleMenu} plain />
    {
      openMenu && (
        <Layer position="center" margin="medium" responsive full="vertical" modal onClickOutside={toggleMenu} onEsc={toggleMenu}>
          <Box gap='large' align='center' pad='medium'>
            <Close onClick={toggleMenu} size='small' />
            <BoxClaim isLogged={isLogged} unclaimedAmount={unclaimedAmount} handleClaim={handleClaim} />
            <Button disabled={isLogged ? false : true} label='my history moves' size='small' margin={{ horizontal: 'large' }} onClick={toggleHistory} />
            <ConnectBox size={size} accountId={accountId} wallet={wallet} isLogged={isLogged} explorerUrl={explorerUrl} />
            <Select
              id="select"
              size='small'
              name="select"
              value={<Box pad={{ left: 'small', vertical: 'xsmall' }}><Text size='small'>{themeMode}</Text></Box>}
              options={['dark', 'light']}
              onChange={({ option }) => {
                console.log(option)
                appState.themeMode = option
              }}
            >
              {
                (option, _) => (
                  <Box pad={{ left: 'small', vertical: 'xsmall' }}>
                    <Text size='small'>{option}</Text>
                  </Box>
                )}
            </Select>
          </Box>
        </Layer>
      )}
  </Box>
}

const BoxClaim = ({ isLogged, handleClaim, unclaimedAmount }) => {
  return (<Box pad='xsmall' align='center' border={{color: 'c2'}}>
    <Text size='small' weight='bold'>unclaimed:</Text>
    <Text size='small' weight='bold'>{utils.format.formatNearAmount(unclaimedAmount)} NEAR</Text>
    <Button disabled={isLogged && unclaimedAmount !== '0' ? false : true} label='claim' size='small' onClick={handleClaim} />
  </Box>)
}

const Nav = () => {
  const size = useContext(ResponsiveContext);
  const { wallet, accountId, isLogged, explorerUrl, unclaimedAmount, contract, themeMode } = useSnapshot(appState);
  const [openHistory, setOpenHistory] = React.useState(false);
  const toggleHistory = () => setOpenHistory((value) => !value);

  const handleClaim = () => {
    contract.withdraw().then(resp => console.log(resp))
  }
  return <Header background='c4' pad={{ horizontal: "2em", vertical: "3em" }} margin={{ top: 'small' }} height="xsmall">
    <Anchor size='large' href="#" label="GawiBawiBo" />
    <Box>
      {
        openHistory && (
          <Layer onClickOutside={toggleHistory} onEsc={toggleHistory}>
            <Box align='center' pad='small' gap='small'>
              <Close onClick={toggleHistory} size='small' />
              <History />
            </Box>
          </Layer>
        )
      }
      {
        size === 'small' ? (
          <ResponsiveMenu toggleHistory={toggleHistory} size={size} accountId={accountId} wallet={wallet} isLogged={isLogged} unclaimedAmount={unclaimedAmount} themeMode={themeMode} handleClaim={handleClaim}/>
        ) : (
          <Box gap={size !== 'large' ? 'small' : 'medium'} direction='row' align='center'>
            <BoxClaim isLogged={isLogged} handleClaim={handleClaim} unclaimedAmount={unclaimedAmount} />
            <Button disabled={isLogged ? false : true} label='my history moves' size='small' onClick={toggleHistory} margin={{ horizontal: size === 'medium' ? 'small' : 'medium' }} />
            <ConnectBox size={size} accountId={accountId} wallet={wallet} isLogged={isLogged} explorerUrl={explorerUrl} />
            <Select
              id="select"
              size='small'
              name="select"
              value={<Box pad={{ left: 'small', vertical: 'xsmall' }}><Text size='small'>{themeMode}</Text></Box>}
              options={['dark', 'light']}
              onChange={({ option }) => {
                console.log(option)
                appState.themeMode = option
              }}
            >
              {
                (option, _) => (
                  <Box pad={{ left: 'small', vertical: 'xsmall' }}>
                    <Text size='small'>{option}</Text>
                  </Box>
                )}
            </Select>
          </Box>
        )
      }

    </Box>
  </Header >;
};

export default Nav;
