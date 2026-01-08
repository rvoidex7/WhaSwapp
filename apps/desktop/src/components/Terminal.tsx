import { useTerminalStore } from '../stores/terminalStore';
import { useEffect, useRef } from 'react';

export default function Terminal() {
  const logs = useTerminalStore((state) => state.logs);
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  return (
    <div className="h-full flex flex-col bg-black text-green-500 font-mono text-xs p-4 overflow-hidden">
      <div className="flex-1 overflow-y-auto">
        {logs.map((log, i) => (
          <div key={i} className="mb-1">
            <span className="text-gray-500">[{log.timestamp}]</span>{' '}
            <span className={log.level === 'ERROR' ? 'text-red-500' : 'text-blue-400'}>
              {log.source}:
            </span>{' '}
            <span className={log.level === 'ERROR' ? 'text-red-400' : 'text-green-400'}>
              {log.message}
            </span>
          </div>
        ))}
        <div ref={endRef} />
      </div>
    </div>
  );
}
