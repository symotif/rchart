<script lang="ts">
	import TabList from '../../lib/components/TabList.svelte';
	import { goto } from '$app/navigation';
	import { addTab, setActiveTab, TabStore, ActiveTabStore } from '../../stores/TabStore';

	let comp: TabList;

	let data = $state([
		{
			number: '001',
			pic: '',
			first_name: 'Logan',
			last_name: 'Nguyen',
			DOB: '9/15/98',
			Age: '24',
			Sex: 'M',
			Gender: 'M',
			Address: '222 Main St. Detroit',
			'Phone Number': '555-555-5555',
			Email: 'lg@asd.com',
			'Last Seen': '5/5/23',
			PCP: 'Dr. Geedee',
			HealthScore: '1'
		},
		{
			number: '001',
			pic: '',
			first_name: 'Sean',
			last_name: 'Nguyen',
			DOB: '9/15/98',
			Age: '24',
			Sex: 'M',
			Gender: 'M',
			Address: '222 Main St. Detroit',
			'Phone Number': '555-555-5555',
			Email: 'lg@asd.com',
			'Last Seen': '5/5/23',
			PCP: 'Dr. Geedee',
			HealthScore: '2'
		},
		{
			number: '001',
			pic: '',
			first_name: 'Evan',
			last_name: 'Nguyen',
			DOB: '9/15/98',
			Age: '24',
			Sex: 'M',
			Gender: 'M',
			Address: '222 Main St. Detroit',
			'Phone Number': '555-555-5555',
			Email: 'lg@asd.com',
			'Last Seen': '5/5/23',
			PCP: 'Dr. Geedee',
			HealthScore: '4'
		},
		{
			number: '001',
			pic: '',
			first_name: 'Don',
			last_name: 'Nguyen',
			DOB: '9/15/98',
			Age: '24',
			Sex: 'M',
			Gender: 'M',
			Address: '222 Main St. Detroit',
			'Phone Number': '555-555-5555',
			Email: 'lg@asd.com',
			'Last Seen': '5/5/23',
			PCP: 'Dr. Geedee',
			HealthScore: '3'
		},
		{
			number: '001',
			pic: '',
			first_name: 'Mark',
			last_name: 'Nguyen',
			DOB: '9/15/98',
			Age: '24',
			Sex: 'M',
			Gender: 'M',
			Address: '222 Main St. Detroit',
			'Phone Number': '555-555-5555',
			Email: 'lg@asd.com',
			'Last Seen': '5/5/23',
			PCP: 'Dr. Geedee',
			HealthScore: '2'
		},
		{
			number: '001',
			pic: '',
			first_name: 'Maxine',
			last_name: 'Nguyen',
			DOB: '9/15/98',
			Age: '24',
			Sex: 'M',
			Gender: 'M',
			Address: '222 Main St. Detroit',
			'Phone Number': '555-555-5555',
			Email: 'lg@asd.com',
			'Last Seen': '5/5/23',
			PCP: 'Dr. Geedee',
			HealthScore: '2'
		}
	]);

	function onPatientRowClick(patientId: string, patientName: string): void {
		const newTab = {
			id: patientId,
			title: `Patient: ${patientName}`
		};
		addTab(newTab);
		setActiveTab(patientId); // Make sure the clicked tab is active
		goto(`/patient/${patientId}`); // Navigate to the patient's page
	}
</script>

<TabList bind:this={comp} />
<div class="">
	<table class="overflow-hidden absolute left-20 top-20 mx-5 my-8 rounded-lg shadow w-[92%]">
		<thead class="bg-gray-50 border-b-2 border-gray-200">
			<tr>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">#</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">pic</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">First Name</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Last Name</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">DOB</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Age</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Sex</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Gender</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Address</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Phone Number</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Email</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Last Seen</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Physician</th>
				<th class="p-3 text-sm font-semibold tracking-wide text-left">Health Score</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-gray-100">
			{#each data as patient}
				<tr
					onclick={() =>
						onPatientRowClick(
							patient.number,
							`${patient.first_name} ${patient.last_name}`
						)}
					class="bg-white cursor-pointer hover:bg-gray-100 transition-colors"
				>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.number}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap" />
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.first_name}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.last_name}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.DOB}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.Age}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.Sex}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.Gender}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.Address}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient['Phone Number']}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.Email}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient['Last Seen']}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.PCP}</td
					>
					<td class="px-3 py-5 text-sm text-left text-gray-700 whitespace-nowrap"
						>{patient.HealthScore}</td
					>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
