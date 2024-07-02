'use client';

import { useState } from 'react';
import { AppHero } from '../ui/ui-layout';
import { ExplainerUiModal } from './cluster-ui';
import { ClusterUiTable } from './cluster-ui';

export default function ClusterFeature() {
  const [showModal, setShowModal] = useState(false);

  return (
    <div>
      <AppHero
        title="Game History"
        subtitle="Global, and Round information only. For user bets see User Accounts."
      >
        <ExplainerUiModal
          show={showModal}
          hideModal={() => setShowModal(false)}
        />
        <button
          className="btn btn-xs lg:btn-md btn-primary"
          onClick={() => setShowModal(true)}
        >
          How the Game Works
        </button>
      </AppHero>
      <ClusterUiTable />
    </div>
  );
}
