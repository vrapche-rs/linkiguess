<script setup lang="ts">
import ky from "ky";
import { onMounted, ref, type Ref } from "vue";

type LinkResult = {
  data: {
    short: string;
    long: string;
    count: number;
  }[];
};

const links: Ref<LinkResult | null> = ref(null);

const loadLinks = async () => {
  try {
    const res = await ky.get("/api/link", {
      throwHttpErrors: true,
    });

    links.value = await res.json();
  } catch (err) {
    console.error(`error while fetching the links, ${err}`);
  }
};

onMounted(loadLinks());
</script>

<template>
  <div class="w-full py-5 px-[25%] flex items-center">
    <div class="flex flex-col gap-5">
      <div class="flex flex-col gap-2">
        <p>
          This is my experiment with implementing EDA, Event-Driven
          Architecture, in Podman's Kubernetes orchestration. You can find the
          diagram of this system in the repo's README.md.
        </p>
        <p>
          The functionality of this system is minimalistic, as the core motive
          is showing backend skills. The website is just to visualize the links,
          and the backend services perform a bare function.
        </p>
      </div>
      <div v-if="links && links.data.length > 0">
        <table class="w-full border border-gray-300 rounded-lg overflow-hidden">
          <thead class="bg-gray-100 border-b border-gray-300">
            <tr>
              <th
                class="px-4 py-2 text-left font-semibold text-gray-700 w-[18%]"
              >
                Short Link
              </th>
              <th class="px-4 py-2 text-left font-semibold text-gray-700">
                Long Link
              </th>
              <th
                class="px-4 py-2 text-right font-semibold text-gray-700 w-[1%] whitespace-nowrap"
              >
                Clicks
              </th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="entry in links.data"
              :key="entry.short"
              class="border-b border-gray-200 hover:bg-gray-50"
            >
              <td class="px-4 py-2 font-medium text-gray-900">
                {{ entry.short }}
              </td>
              <td class="px-4 py-2 text-gray-800 break-all">
                {{ entry.long }}
              </td>
              <td class="px-4 py-2 text-right font-bold text-gray-700">
                {{ entry.count }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-else>
        <p>No links are available</p>
      </div>
    </div>
  </div>
</template>
