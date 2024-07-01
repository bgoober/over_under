'use client';

import { useConnection } from '@solana/wallet-adapter-react';
import { IconAddressBook, IconTrash } from '@tabler/icons-react';
import { useQuery } from '@tanstack/react-query';
import { ReactNode, useState } from 'react';
import { AppModal } from '../ui/ui-layout';
import { ClusterNetwork, useCluster } from './cluster-data-access';
import { Connection } from '@solana/web3.js';
import ReactMarkdown from 'react-markdown';


export function ExplorerLink({
  path,
  label,
  className,
}: {
  path: string;
  label: string;
  className?: string;
}) {
  const { getExplorerUrl } = useCluster();
  return (
    <a
      href={getExplorerUrl(path)}
      target="_blank"
      rel="noopener noreferrer"
      className={className ? className : `link font-mono`}
    >
      {label}
    </a>
  );
}

export function ClusterChecker({ children }: { children: ReactNode }) {
  const { cluster } = useCluster();
  const { connection } = useConnection();

  const query = useQuery({
    queryKey: ['version', { cluster, endpoint: connection.rpcEndpoint }],
    queryFn: () => connection.getVersion(),
    retry: 1,
  });
  if (query.isLoading) {
    return null;
  }
  if (query.isError || !query.data) {
    return (
      <div className="alert alert-warning text-warning-content/80 rounded-none flex justify-center">
        <span>
          Error connecting to cluster <strong>{cluster.name}</strong>
        </span>
        <button
          className="btn btn-xs btn-neutral"
          onClick={() => query.refetch()}
        >
          Refresh
        </button>
      </div>
    );
  }
  return children;
}

export function ClusterUiSelect() {
  const { clusters, setCluster, cluster } = useCluster();
  return (
    <div className="dropdown dropdown-end">
      <label tabIndex={0} className="btn btn-primary rounded-btn">
        {cluster.name}
      </label>
      <ul
        tabIndex={0}
        className="menu dropdown-content z-[1] p-2 shadow bg-base-100 rounded-box w-52 mt-4"
      >
        {clusters.map((item) => (
          <li key={item.name}>
            <button
              className={`btn btn-sm ${
                item.active ? 'btn-primary' : 'btn-ghost'
              }`}
              onClick={() => setCluster(item)}
            >
              {item.name}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}

// for the global button modal
export function ExplainerUiModal({
  hideModal,
  show,
}: {
  hideModal: () => void;
  show: boolean;
}) {
  const markdown = `
  # Over / Under

  ### 1. A random number 0-1000 is generated every Round.
  ### 2. Players bet on whether the outcome of current Round's Random Number will be:
  -- higher than (over) the previous Round's Random Number. --

                            or                                
  -- lower than (under) the previous Round's Random Number. --
  ### 3. Losers pay Winners.
  ### 4. player_winnings = (player_bet / winning_bets_sum) * total_pot
  ### 5. If the random number is the same as the previous number, the House wins the entire pot. 
  ### 6. This is the House's only win condition and there are no fees taken by the House in any other form.

  [Link to GitHub](https://github.com/bgoober/over_under)
  `;

  return (
    <AppModal
      title="Explainer"
      hide={hideModal}
      show={show}
    >
      <ReactMarkdown>{markdown}</ReactMarkdown>
    </AppModal>
  );
}

// not used
export function ClusterUiModal({
  hideModal,
  show,
}: {
  hideModal: () => void;
  show: boolean;
}) {
  const { addCluster } = useCluster();
  const [name, setName] = useState('');
  const [network, setNetwork] = useState<ClusterNetwork | undefined>();
  const [endpoint, setEndpoint] = useState('');

  return (
    <AppModal
      title={'Add Cluster'}
      hide={hideModal}
      show={show}
      submit={() => {
        try {
          new Connection(endpoint);
          if (name) {
            addCluster({ name, network, endpoint });
            hideModal();
          } else {
            console.log('Invalid cluster name');
          }
        } catch {
          console.log('Invalid cluster endpoint');
        }
      }}
      submitLabel="Save"
    >
      <input
        type="text"
        placeholder="Name"
        className="input input-bordered w-full"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
      <input
        type="text"
        placeholder="Endpoint"
        className="input input-bordered w-full"
        value={endpoint}
        onChange={(e) => setEndpoint(e.target.value)}
      />
      <select
        className="select select-bordered w-full"
        value={network}
        onChange={(e) => setNetwork(e.target.value as ClusterNetwork)}
      >
        <option value={undefined}>Select a network</option>
        <option value={ClusterNetwork.Devnet}>Devnet</option>
        <option value={ClusterNetwork.Testnet}>Testnet</option>
        <option value={ClusterNetwork.Mainnet}>Mainnet</option>
      </select>
    </AppModal>
  );
}

// for global
export function ClusterUiTable() {
  const { clusters, setCluster, deleteCluster } = useCluster();
  return (
    <div className="flex flex-wrap justify-around">
      <div className="overflow-x-auto">
        <table className="table border-4 border-separate border-base-300">
          <thead>
            <tr>
              <th>Global Account:</th>
              <th className="text-center">Account</th>
            </tr>
          </thead>
          <tbody>
            {clusters.map((item) => (
              <tr key={item.name} className={item?.active ? 'bg-base-200' : ''}>
                <td className="space-y-2">
                  <div className="whitespace-nowrap space-x-2">
                    <span className="text-xl">
                      {item?.active ? (
                        item.name
                      ) : (
                        <button
                          title="Select cluster"
                          className="link link-secondary"
                          onClick={() => setCluster(item)}
                        >
                          {item.name}
                        </button>
                      )}
                    </span>
                  </div>
                  {/* <span className="text-xs">
                    Network: {item.network ?? 'custom'}
                  </span> */}
                  {/* <div className="whitespace-nowrap text-gray-500 text-xs">
                    {item.endpoint}
                  </div> */}
                </td>
                <td className="space-x-2 whitespace-nowrap text-center">
                  <button
                    disabled={item?.active}
                    className="btn btn-xs btn-default btn-outline"
                    onClick={() => {
                      if (!window.confirm('Are you sure?')) return;
                      deleteCluster(item);
                    }} 
                  >
                    <IconAddressBook size={16} />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      <div className="overflow-x-auto">
        <table className="table border-4 border-separate border-base-300">
          <thead>
            <tr>
              <th>Round Accounts:</th>
              <th className="text-center">Accounts</th>
            </tr>
          </thead>
          <tbody>
            {clusters.map((item) => (
              <tr key={item.name} className={item?.active ? 'bg-base-200' : ''}>
                <td className="space-y-2">
                  <div className="whitespace-nowrap space-x-2">
                    <span className="text-xl">
                      {item?.active ? (
                        item.name
                      ) : (
                        <button
                          title="Select cluster"
                          className="link link-secondary"
                          onClick={() => setCluster(item)}
                        >
                          {item.name}
                        </button>
                      )}
                    </span>
                  </div>
                  {/* <span className="text-xs">
                    Network: {item.network ?? 'custom'}
                  </span> */}
                  {/* <div className="whitespace-nowrap text-gray-500 text-xs">
                    {item.endpoint}
                  </div> */}
                </td>
                <td className="space-x-2 whitespace-nowrap text-center">
                  <button
                    disabled={item?.active}
                    className="btn btn-xs btn-default btn-outline"
                    onClick={() => {
                      if (!window.confirm('Are you sure?')) return;
                      deleteCluster(item);
                    }}
                  >
                    <IconAddressBook size={16} />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}