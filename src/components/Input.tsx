import { emit, listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

export function Input() {
	const [commands, setCommands] = useState<string[]>([])

	useEffect(() => {
		invoke('init_process')
		listen('event-name', (event) => {
			setCommands(state => [...state, event.payload.message])
		})
	}, [])

	return (
		<main className='h-full bg-stone-900 text-white p-4 overflow-auto'>
			<ul>
				{commands.map(command => <li>{command}</li>)}
			</ul>
		</main>
	)
}
