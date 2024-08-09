<template>
  <v-table>
    <thead>
      <tr>
        <th class="text-left">
          URL
        </th>
        <th class="text-left">
          Status
        </th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="item in downloads"
        :key="item.name"
      >
        <td class="text-left">{{ item.url }}</td>
        <td class="text-left">{{ item.status }}</td>
      </tr>
    </tbody>
  </v-table>
</template>

<script>
  export default {
    data () {
      return {
        downloads: [
          {
            url: 'Frozen Yogurt',
            status: 159,
          },
        ],
      }
    },
  }
</script>
