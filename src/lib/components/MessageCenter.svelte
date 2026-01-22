<script lang="ts">
	let isOpen = $state(false);
	let activeTab = $state<'staff' | 'patients'>('staff');
	let doNotDisturb = $state(false);
	let searchQuery = $state('');
	let selectedChatId = $state<string | null>(null);
	let messageInput = $state('');
	let draggedChatId = $state<string | null>(null);
	let dragOverGroupId = $state<string | null>(null);

	// Types
	type Chat = {
		id: string;
		name: string;
		initials: string;
		avatarColor: string;
		avatarUrl?: string;
		lastMessage: string;
		timestamp: string;
		unread: number;
		isPinned: boolean;
		isOnline?: boolean;
		isGroup?: boolean;
		groupId?: string;
		type: 'staff' | 'patient';
		members?: { name: string; initials: string; avatarColor: string }[];
	};

	type ChatGroup = {
		id: string;
		name: string;
		isExpanded: boolean;
	};

	type Message = {
		id: string;
		senderId: string;
		senderName: string;
		content: string;
		timestamp: string;
		isOwn: boolean;
	};

	type Consultant = {
		id: string;
		name: string;
		initials: string;
		specialty: string;
		avatarColor: string;
		isOnline: boolean;
	};

	// Sample data - Staff chats
	let staffChats = $state<Chat[]>([
		{ id: 's1', name: 'Dr. Sarah Chen', initials: 'SC', avatarColor: 'bg-blue-500', lastMessage: 'The labs are back for room 204', timestamp: '2m ago', unread: 2, isPinned: true, isOnline: true, type: 'staff' },
		{ id: 's2', name: 'Dr. Michael Ross', initials: 'MR', avatarColor: 'bg-green-500', lastMessage: 'Can you cover my 3pm?', timestamp: '15m ago', unread: 0, isPinned: true, isOnline: true, type: 'staff' },
		{ id: 's3', name: 'Nursing Station 3A', initials: 'N3', avatarColor: 'bg-purple-500', lastMessage: 'Patient in 312 needs attention', timestamp: '1h ago', unread: 5, isPinned: false, isOnline: true, type: 'staff', isGroup: true, members: [{ name: 'Nurse Amy', initials: 'NA', avatarColor: 'bg-pink-500' }, { name: 'Nurse Bob', initials: 'NB', avatarColor: 'bg-indigo-500' }] },
		{ id: 's4', name: 'Dr. Emily Watson', initials: 'EW', avatarColor: 'bg-pink-500', lastMessage: 'Thanks for the consult!', timestamp: '2h ago', unread: 0, isPinned: false, isOnline: false, type: 'staff' },
		{ id: 's5', name: 'Cardiology Team', initials: 'CT', avatarColor: 'bg-red-500', lastMessage: 'Echo scheduled for tomorrow', timestamp: '3h ago', unread: 0, isPinned: false, isOnline: true, type: 'staff', isGroup: true, members: [{ name: 'Dr. Heart', initials: 'DH', avatarColor: 'bg-red-400' }, { name: 'Dr. Pulse', initials: 'DP', avatarColor: 'bg-red-600' }] },
	]);

	// Sample data - Patient chats
	let patientChats = $state<Chat[]>([
		{ id: 'p1', name: 'Logan Smith', initials: 'LS', avatarColor: 'bg-orange-500', lastMessage: 'When should I take my new medication?', timestamp: '30m ago', unread: 1, isPinned: false, type: 'patient' },
		{ id: 'p2', name: 'John Doe', initials: 'JD', avatarColor: 'bg-teal-500', lastMessage: 'Thank you doctor!', timestamp: '1h ago', unread: 0, isPinned: false, type: 'patient' },
		{ id: 'p3', name: 'Max Johnson', initials: 'MJ', avatarColor: 'bg-cyan-500', lastMessage: 'I have a question about my results', timestamp: '2h ago', unread: 3, isPinned: true, type: 'patient' },
	]);

	// Chat groups
	let chatGroups = $state<ChatGroup[]>([
		{ id: 'cardiology', name: 'Cardiology', isExpanded: true },
		{ id: 'nursing', name: 'Nursing', isExpanded: true },
		{ id: 'general', name: 'General', isExpanded: true },
	]);

	// Favorite consultants
	let favoriteConsultants = $state<Consultant[]>([
		{ id: 'c1', name: 'Dr. James Park', initials: 'JP', specialty: 'Cardiology', avatarColor: 'bg-red-500', isOnline: true },
		{ id: 'c2', name: 'Dr. Lisa Wong', initials: 'LW', specialty: 'Neurology', avatarColor: 'bg-purple-500', isOnline: false },
		{ id: 'c3', name: 'Dr. Robert Kim', initials: 'RK', specialty: 'Oncology', avatarColor: 'bg-green-500', isOnline: true },
	]);

	// Sample messages for selected chat
	let messages = $state<Message[]>([
		{ id: 'm1', senderId: 's1', senderName: 'Dr. Sarah Chen', content: 'Hi, the labs are back for room 204', timestamp: '10:30 AM', isOwn: false },
		{ id: 'm2', senderId: 'me', senderName: 'You', content: 'Thanks! What are the results?', timestamp: '10:32 AM', isOwn: true },
		{ id: 'm3', senderId: 's1', senderName: 'Dr. Sarah Chen', content: 'BMP looks good, but potassium is slightly elevated at 5.2', timestamp: '10:33 AM', isOwn: false },
		{ id: 'm4', senderId: 'me', senderName: 'You', content: 'Got it. I\'ll adjust the medications accordingly.', timestamp: '10:35 AM', isOwn: true },
	]);

	const toggleDrawer = () => {
		isOpen = !isOpen;
	};

	function toggleDND() {
		doNotDisturb = !doNotDisturb;
	}

	function selectChat(chatId: string) {
		selectedChatId = chatId;
	}

	function togglePin(chatId: string, type: 'staff' | 'patient') {
		if (type === 'staff') {
			staffChats = staffChats.map(c => c.id === chatId ? { ...c, isPinned: !c.isPinned } : c);
		} else {
			patientChats = patientChats.map(c => c.id === chatId ? { ...c, isPinned: !c.isPinned } : c);
		}
	}

	function sendMessage() {
		if (!messageInput.trim() || !selectedChatId) return;

		const newMessage: Message = {
			id: `m${Date.now()}`,
			senderId: 'me',
			senderName: 'You',
			content: messageInput,
			timestamp: new Date().toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' }),
			isOwn: true
		};

		messages = [...messages, newMessage];
		messageInput = '';
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			sendMessage();
		}
	}

	function handleDragStart(chatId: string) {
		draggedChatId = chatId;
	}

	function handleDragOver(e: DragEvent, groupId: string) {
		e.preventDefault();
		dragOverGroupId = groupId;
	}

	function handleDrop(groupId: string) {
		if (draggedChatId) {
			staffChats = staffChats.map(c =>
				c.id === draggedChatId ? { ...c, groupId } : c
			);
		}
		draggedChatId = null;
		dragOverGroupId = null;
	}

	function toggleGroupExpand(groupId: string) {
		chatGroups = chatGroups.map(g =>
			g.id === groupId ? { ...g, isExpanded: !g.isExpanded } : g
		);
	}

	// Filtered chats based on search
	let filteredStaffChats = $derived(
		searchQuery
			? staffChats.filter(c => c.name.toLowerCase().includes(searchQuery.toLowerCase()))
			: staffChats
	);

	let filteredPatientChats = $derived(
		searchQuery
			? patientChats.filter(c => c.name.toLowerCase().includes(searchQuery.toLowerCase()))
			: patientChats
	);

	// Sort chats - pinned first, then by timestamp
	let sortedStaffChats = $derived(
		[...filteredStaffChats].sort((a, b) => {
			if (a.isPinned && !b.isPinned) return -1;
			if (!a.isPinned && b.isPinned) return 1;
			return 0;
		})
	);

	let sortedPatientChats = $derived(
		[...filteredPatientChats].sort((a, b) => {
			if (a.isPinned && !b.isPinned) return -1;
			if (!a.isPinned && b.isPinned) return 1;
			return 0;
		})
	);

	// Get current chat
	let currentChat = $derived(
		activeTab === 'staff'
			? staffChats.find(c => c.id === selectedChatId)
			: patientChats.find(c => c.id === selectedChatId)
	);

	// Total unread count
	let totalUnread = $derived(
		staffChats.reduce((sum, c) => sum + c.unread, 0) +
		patientChats.reduce((sum, c) => sum + c.unread, 0)
	);
</script>

<!-- Invisible click-away area -->
{#if isOpen}
	<div
		class="fixed inset-0 z-40"
		onclick={toggleDrawer}
		onkeydown={(e) => e.key === 'Escape' && toggleDrawer()}
		role="button"
		tabindex="0"
		aria-label="Close message center"
	></div>
{/if}

<!-- Drawer -->
<div
	class="fixed top-20 h-[calc(100%-6rem)] bg-gray-100 dark:bg-gray-800 shadow-xl rounded-l-lg transition-all duration-300 ease-in-out flex flex-row z-50"
	style="width: 800px; right: {isOpen ? '0px' : '-750px'};"
>
	<!-- Pull Tab - positioned outside the drawer, aligned with edge when closed -->
	<div class="absolute -left-12 top-1/2 -translate-y-1/2 z-50">
		<button
			class="-rotate-90 px-5 py-2.5 bg-blue-500 dark:bg-blue-600 text-white rounded-t-lg cursor-pointer whitespace-nowrap shadow-lg hover:bg-blue-600 dark:hover:bg-blue-700 transition-colors flex items-center gap-2"
			onclick={toggleDrawer}
		>
			<i class="fa-solid fa-comments"></i>
			<span class="text-sm font-bold">Messages</span>
			{#if totalUnread > 0 && !doNotDisturb}
				<span class="bg-red-500 text-white text-xs rounded-full px-1.5 py-0.5 min-w-[1.25rem] text-center">
					{totalUnread > 99 ? '99+' : totalUnread}
				</span>
			{/if}
		</button>
	</div>

	<!-- Drawer Handle Bar -->
	<button
		class="w-3 flex-shrink-0 bg-gray-200 dark:bg-gray-700 cursor-pointer hover:bg-blue-300 dark:hover:bg-blue-600 transition-colors border-0"
		onclick={toggleDrawer}
		aria-label="Toggle message center"
	></button>

	<!-- Main Content -->
	<div class="flex flex-1 h-full overflow-hidden">
		<!-- Left Panel: Chat List -->
		<div class="w-64 flex flex-col border-r border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900">
			<!-- Header with DND -->
			<div class="p-3 border-b border-gray-200 dark:border-gray-700">
				<div class="flex items-center justify-between mb-3">
					<h2 class="text-lg font-semibold text-gray-800 dark:text-gray-100">Messages</h2>
					<button
						onclick={toggleDND}
						class="p-2 rounded-lg transition-colors {doNotDisturb ? 'bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400' : 'hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-500 dark:text-gray-400'}"
						title={doNotDisturb ? 'Do Not Disturb ON' : 'Do Not Disturb OFF'}
					>
						<i class="fa-solid {doNotDisturb ? 'fa-bell-slash' : 'fa-bell'}"></i>
					</button>
				</div>

				<!-- Search -->
				<div class="relative">
					<i class="fa-solid fa-search absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm"></i>
					<input
						type="text"
						bind:value={searchQuery}
						placeholder="Search people..."
						class="w-full pl-9 pr-3 py-2 text-sm bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800 dark:text-gray-200"
					/>
				</div>
			</div>

			<!-- Staff/Patient Tabs -->
			<div class="flex border-b border-gray-200 dark:border-gray-700">
				<button
					onclick={() => { activeTab = 'staff'; selectedChatId = null; }}
					class="flex-1 py-2 text-sm font-medium transition-colors {activeTab === 'staff' ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400' : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200'}"
				>
					<i class="fa-solid fa-user-doctor mr-1"></i>
					Staff
				</button>
				<button
					onclick={() => { activeTab = 'patients'; selectedChatId = null; }}
					class="flex-1 py-2 text-sm font-medium transition-colors {activeTab === 'patients' ? 'text-orange-600 dark:text-orange-400 border-b-2 border-orange-600 dark:border-orange-400' : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200'}"
				>
					<i class="fa-solid fa-hospital-user mr-1"></i>
					Patients
				</button>
			</div>

			<!-- Patient Warning Border -->
			{#if activeTab === 'patients'}
				<div class="mx-2 mt-2 p-2 bg-orange-50 dark:bg-orange-900/20 border-2 border-orange-400 dark:border-orange-600 rounded-lg">
					<p class="text-xs text-orange-700 dark:text-orange-300 text-center font-medium">
						<i class="fa-solid fa-exclamation-triangle mr-1"></i>
						Patient Messaging Mode
					</p>
				</div>
			{/if}

			<!-- Chat List -->
			<div class="flex-1 overflow-y-auto p-2 space-y-1">
				{#if activeTab === 'staff'}
					{#each sortedStaffChats as chat (chat.id)}
						<button
							draggable="true"
							ondragstart={() => handleDragStart(chat.id)}
							onclick={() => selectChat(chat.id)}
							class="w-full p-2 rounded-lg transition-colors text-left flex items-center gap-2 {selectedChatId === chat.id ? 'bg-blue-100 dark:bg-blue-900/30' : 'hover:bg-gray-100 dark:hover:bg-gray-800'}"
						>
							<!-- Avatar -->
							<div class="relative flex-shrink-0">
								{#if chat.isGroup && chat.members}
									<div class="w-10 h-10 relative">
										<div class="absolute top-0 left-0 w-7 h-7 rounded-full {chat.members[0].avatarColor} flex items-center justify-center text-white text-xs font-bold border-2 border-white dark:border-gray-900">
											{chat.members[0].initials}
										</div>
										<div class="absolute bottom-0 right-0 w-7 h-7 rounded-full {chat.members[1].avatarColor} flex items-center justify-center text-white text-xs font-bold border-2 border-white dark:border-gray-900">
											{chat.members[1].initials}
										</div>
									</div>
								{:else}
									<div class="w-10 h-10 rounded-full {chat.avatarColor} flex items-center justify-center text-white font-bold text-sm">
										{chat.initials}
									</div>
								{/if}
								{#if chat.isOnline}
									<div class="absolute bottom-0 right-0 w-3 h-3 bg-green-500 rounded-full border-2 border-white dark:border-gray-900"></div>
								{/if}
							</div>

							<!-- Content -->
							<div class="flex-1 min-w-0">
								<div class="flex items-center justify-between">
									<span class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">{chat.name}</span>
									<span class="text-xs text-gray-400 flex-shrink-0">{chat.timestamp}</span>
								</div>
								<p class="text-xs text-gray-500 dark:text-gray-400 truncate">{chat.lastMessage}</p>
							</div>

							<!-- Badges -->
							<div class="flex flex-col items-center gap-1 flex-shrink-0">
								{#if chat.isPinned}
									<i class="fa-solid fa-thumbtack text-blue-500 text-xs"></i>
								{/if}
								{#if chat.unread > 0}
									<span class="bg-blue-500 text-white text-xs rounded-full px-1.5 py-0.5 min-w-[1.25rem] text-center">
										{chat.unread}
									</span>
								{/if}
							</div>
						</button>
					{/each}
				{:else}
					{#each sortedPatientChats as chat (chat.id)}
						<button
							onclick={() => selectChat(chat.id)}
							class="w-full p-2 rounded-lg transition-colors text-left flex items-center gap-2 border-l-4 {selectedChatId === chat.id ? 'bg-orange-100 dark:bg-orange-900/30 border-orange-500' : 'hover:bg-gray-100 dark:hover:bg-gray-800 border-transparent'}"
						>
							<div class="w-10 h-10 rounded-full {chat.avatarColor} flex items-center justify-center text-white font-bold text-sm flex-shrink-0">
								{chat.initials}
							</div>
							<div class="flex-1 min-w-0">
								<div class="flex items-center justify-between">
									<span class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">{chat.name}</span>
									<span class="text-xs text-gray-400 flex-shrink-0">{chat.timestamp}</span>
								</div>
								<p class="text-xs text-gray-500 dark:text-gray-400 truncate">{chat.lastMessage}</p>
							</div>
							<div class="flex flex-col items-center gap-1 flex-shrink-0">
								{#if chat.isPinned}
									<i class="fa-solid fa-thumbtack text-orange-500 text-xs"></i>
								{/if}
								{#if chat.unread > 0}
									<span class="bg-orange-500 text-white text-xs rounded-full px-1.5 py-0.5 min-w-[1.25rem] text-center">
										{chat.unread}
									</span>
								{/if}
							</div>
						</button>
					{/each}
				{/if}
			</div>

			<!-- Favorite Consultants -->
			<div class="border-t border-gray-200 dark:border-gray-700 p-2">
				<p class="text-xs font-medium text-gray-500 dark:text-gray-400 mb-2 px-1">Favorite Consultants</p>
				<div class="flex gap-2 overflow-x-auto pb-1">
					{#each favoriteConsultants as consultant (consultant.id)}
						<button
							class="flex flex-col items-center gap-1 p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors flex-shrink-0"
							title="{consultant.name} - {consultant.specialty}"
						>
							<div class="relative">
								<div class="w-8 h-8 rounded-full {consultant.avatarColor} flex items-center justify-center text-white text-xs font-bold">
									{consultant.initials}
								</div>
								{#if consultant.isOnline}
									<div class="absolute bottom-0 right-0 w-2.5 h-2.5 bg-green-500 rounded-full border-2 border-white dark:border-gray-900"></div>
								{/if}
							</div>
							<span class="text-xs text-gray-600 dark:text-gray-400 truncate max-w-[50px]">{consultant.name.split(' ')[1]}</span>
						</button>
					{/each}
					<button
						class="flex flex-col items-center justify-center gap-1 p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors flex-shrink-0"
						title="Add consultant"
					>
						<div class="w-8 h-8 rounded-full bg-gray-200 dark:bg-gray-700 flex items-center justify-center text-gray-500 dark:text-gray-400">
							<i class="fa-solid fa-plus text-xs"></i>
						</div>
						<span class="text-xs text-gray-500 dark:text-gray-400">Add</span>
					</button>
				</div>
			</div>
		</div>

		<!-- Right Panel: Chat Area -->
		<div class="flex-1 flex flex-col bg-gray-50 dark:bg-gray-800 {activeTab === 'patients' ? 'border-2 border-orange-400 dark:border-orange-600 rounded-r-lg' : ''}">
			{#if selectedChatId && currentChat}
				<!-- Chat Header -->
				<div class="p-3 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 flex items-center justify-between">
					<div class="flex items-center gap-3">
						{#if currentChat.isGroup && currentChat.members}
							<div class="w-10 h-10 relative">
								<div class="absolute top-0 left-0 w-7 h-7 rounded-full {currentChat.members[0].avatarColor} flex items-center justify-center text-white text-xs font-bold border-2 border-white dark:border-gray-900">
									{currentChat.members[0].initials}
								</div>
								<div class="absolute bottom-0 right-0 w-7 h-7 rounded-full {currentChat.members[1].avatarColor} flex items-center justify-center text-white text-xs font-bold border-2 border-white dark:border-gray-900">
									{currentChat.members[1].initials}
								</div>
							</div>
						{:else}
							<div class="w-10 h-10 rounded-full {currentChat.avatarColor} flex items-center justify-center text-white font-bold">
								{currentChat.initials}
							</div>
						{/if}
						<div>
							<h3 class="font-medium text-gray-800 dark:text-gray-100">{currentChat.name}</h3>
							{#if currentChat.isOnline}
								<p class="text-xs text-green-500">Online</p>
							{:else}
								<p class="text-xs text-gray-400">Offline</p>
							{/if}
						</div>
					</div>
					<div class="flex items-center gap-2">
						<button
							onclick={() => togglePin(selectedChatId!, currentChat!.type)}
							class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors {currentChat.isPinned ? 'text-blue-500' : 'text-gray-400'}"
							title={currentChat.isPinned ? 'Unpin chat' : 'Pin chat'}
						>
							<i class="fa-solid fa-thumbtack"></i>
						</button>
						<button
							class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors text-gray-400"
							title="More options"
						>
							<i class="fa-solid fa-ellipsis-v"></i>
						</button>
					</div>
				</div>

				<!-- Messages Area -->
				<div class="flex-1 overflow-y-auto p-4 space-y-4">
					{#each messages as message (message.id)}
						<div class="flex {message.isOwn ? 'justify-end' : 'justify-start'}">
							<div class="max-w-[70%] {message.isOwn ? 'bg-blue-500 text-white' : 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200'} rounded-lg px-4 py-2 shadow-sm">
								{#if !message.isOwn}
									<p class="text-xs font-medium mb-1 {message.isOwn ? 'text-blue-100' : 'text-gray-500 dark:text-gray-400'}">{message.senderName}</p>
								{/if}
								<p class="text-sm">{message.content}</p>
								<p class="text-xs mt-1 {message.isOwn ? 'text-blue-100' : 'text-gray-400'}">{message.timestamp}</p>
							</div>
						</div>
					{/each}
				</div>

				<!-- Message Input -->
				<div class="p-3 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900">
					{#if activeTab === 'patients'}
						<div class="mb-2 p-2 bg-orange-50 dark:bg-orange-900/20 rounded-lg border border-orange-200 dark:border-orange-800">
							<p class="text-xs text-orange-700 dark:text-orange-300">
								<i class="fa-solid fa-exclamation-triangle mr-1"></i>
								You are messaging a patient. Messages may be part of medical record.
							</p>
						</div>
					{/if}
					<div class="flex items-center gap-2">
						<button
							class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg transition-colors text-gray-400"
							title="Attach file"
							aria-label="Attach file"
						>
							<i class="fa-solid fa-paperclip"></i>
						</button>
						<input
							type="text"
							bind:value={messageInput}
							onkeydown={handleKeydown}
							placeholder="Type a message..."
							class="flex-1 px-4 py-2 bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800 dark:text-gray-200 text-sm"
						/>
						<button
							onclick={sendMessage}
							disabled={!messageInput.trim()}
							class="p-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							title="Send message"
							aria-label="Send message"
						>
							<i class="fa-solid fa-paper-plane"></i>
						</button>
					</div>
				</div>
			{:else}
				<!-- No Chat Selected -->
				<div class="flex-1 flex items-center justify-center text-gray-400 dark:text-gray-500">
					<div class="text-center">
						<i class="fa-solid fa-comments text-6xl mb-4 opacity-30"></i>
						<p class="text-lg">Select a conversation</p>
						<p class="text-sm">Choose a chat from the list to start messaging</p>
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

