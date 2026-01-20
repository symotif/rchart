<script>
	import { SideBarStore, setTab } from '../../stores/SideBarStore';
</script>

<div class="group mb-4">
	<div
		class="relative flex items-center justify-center mt-3 transform transition-all duration-200 ease-in-out group-hover:scale-110"
	>
		<!-- profile -->
		<a
			href={'/profile'}
			onclick={() => {
				setTab(-1);
			}}
		>
			<img
				class="w-16 h-16 rounded-full border-2 border-blue-400"
				src="./src/lib/img/doctor_headshot.jpg"
				alt="profile-pic"
				draggable="false"
			/>
		</a>

		<!-- institution -->
		<div>
			<img
				class="w-8 h-8 rounded-full justify-center absolute bottom-0 right-0"
				src="./src/lib/img/wsu.png"
				alt="institution"
				draggable="false"
			/>
			<span class="w-4 h-4 rounded-full bg-green-500 absolute bottom-0 left-2"></span>
		</div>
	</div>

	<!-- tooltip -->
	<span
		class="absolute w-auto p-2 m-2 min-2-max left-20 rounded-md
            shadow-md text-white bg-gray-600 text-xs font-bold
            transition-all duration-100 scale-0 origin-left group-hover:scale-100"
	>
		profile
	</span>
</div>
