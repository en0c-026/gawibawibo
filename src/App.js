import React, { useEffect, useState } from 'react';
import { Grommet, ResponsiveContext, Spinner, Image, Box } from 'grommet';
import { theme } from './layout/theme';
import { keyStores, connect, Near, WalletConnection, Contract } from 'near-api-js';
import { useSnapshot } from 'valtio';
import Nav from './components/Nav';
import Container from './components/Container';
import Main from './components/Main';
import FooterApp from './components/FooterApp';
import { proxy } from 'valtio';
import getConfig from './config';

const initNear = async () => {

  const keyStore = new keyStores.BrowserLocalStorageKeyStore();
  const config = getConfig('testnet');

  const client = await connect({ keyStore, ...config });
  const wallet = new WalletConnection(client, 'gawibawibo');

  return { wallet, client, config };
}

export const appState = proxy({
  env: '',
  themeMode: 'dark',
  explorerUrl: '',
  isLogged: false,
  accountId: '',
  balance: '0',
  unplayedMoves: [],
  unclaimedAmount: '0',
  historyMoves: []
});


function App() {
  const [loading, setLoading] = useState(true)
  const { themeMode} = useSnapshot(appState);
  useEffect(() => {
    initNear().then(({ wallet, config }) => {
      appState.wallet = wallet;
      appState.env = config.networkId;
      appState.explorerUrl = config.explorerUrl;
      if (localStorage.getItem('gawibawibo_wallet_auth_key')) {
        appState.accountId = wallet.getAccountId()
        appState.isLogged = true;
        appState.contract = new Contract(
          wallet.account(),
          'gawibawibo.en0c-test.testnet',
          {
            changeMethods: ['new_move', 'cancel_move', 'withdraw', 'play_move'],
            viewMethods: ['moves_of', 'get_unplayed_moves', 'unclaimed_amount_of'],
          }
        )
        wallet.account().getAccountBalance().then((_balance) => {
          appState.balance = _balance.total;
        })
        appState.contract.get_unplayed_moves().then(resp => appState.unplayedMoves = resp)
        appState.contract.unclaimed_amount_of({ account_id: appState.accountId }).then(resp => appState.unclaimedAmount = resp)
        appState.contract.moves_of({ account_id: appState.accountId }).then(resp => appState.historyMoves = resp)

      }
    })
  }, [])

  useEffect(() => {
    const timer = setTimeout(() => {
      setLoading(false)
    }, 2500);
    return () => clearTimeout(timer);
  }, []);
  
  return (
    <Grommet theme={theme} themeMode={themeMode} background='c1' full>
      <ResponsiveContext.Consumer>
        {size => (
          <Container size={size !== 'large' ? 'small' : 'large'}>
            {
              loading ?
                (
                  <Box align='center' pad={{top: 'xlarge'}} margin={{top: 'xlarge'}} flex='grow'>
                  <Spinner animation={{ type: 'rotateRight', duration: 4000 }} size='xlarge'>
                    <Image src='loading.png' />
                  </Spinner>
                  </Box>
                )
                :
                (
                  <>
                    <Nav />
                    <Main />
                    <FooterApp />
                  </>
                )
            }
          </Container>
        )}
      </ResponsiveContext.Consumer>
    </Grommet>
  );
}

export default App;
