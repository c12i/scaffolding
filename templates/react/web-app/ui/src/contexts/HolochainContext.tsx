import React, { createContext, useEffect, useRef, useState } from 'react';
import { AppWebsocket, type AppClient } from '@holochain/client';

const HolochainContext = createContext<HolochainContextValues>({
  client: undefined,
	error: undefined,
	loading: true,
});

const HolochainClientProvider: React.FC<HolochainClientProviderProps> = ({ children }) => {
  const [loading, setLoading] = useState(true);
	const [error, setError] = useState<unknown>();
  const client = useRef<AppClient>();

  useEffect(() => {
    const connectClient = async () => {
      try {
        client.current = await AppWebsocket.connect();
      } catch (error) {
				setError(error)
        console.error('Failed to establish websocket connection:', error);
      } finally {
        setLoading(false);
      }
    };
    connectClient();
  }, []);

  return (
    <HolochainContext.Provider value={{ client: client.current, error, loading }}>
      {children}
    </HolochainContext.Provider>
  );
};

interface HolochainContextValues {
	client: AppClient | undefined,
	error: unknown | undefined
	loading: boolean,
}

interface HolochainClientProviderProps {
  children: React.ReactNode;
}

export default HolochainClientProvider;
