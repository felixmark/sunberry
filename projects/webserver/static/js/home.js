document.addEventListener('DOMContentLoaded', function() {
  const ctx = document.getElementById('power_chart');
  let chart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: [],
      datasets: [{
        label: 'Power used (W)',
        data: [],
        borderWidth: 3,
        borderColor: "#559598",
        backgroundColor: "#55959840",
        fill: true,
      }, {
        label: 'Photovoltaic power (W)',
        data: [],
        borderWidth: 3,
        borderColor: "#d79921",
        backgroundColor: "#d7992120",
        fill: true,
      }],
    },
    options: {
      interaction: {
        intersect: false,
        mode: 'index',
      },
      scales: {
        y: {
          max: 22,
        },
        x: {
          ticks: {
            maxTicksLimit: 2
          }
        }
      },
      animation: {
        duration: 0
      },
      maintainAspectRatio: false,
      cubicInterpolationMode: 'monotone',
      tension: 0.5,
      pointRadius: 0,
      pointHitRadius: 20,
      plugins: {
        legend: {
          labels: {
            usePointStyle: true,
            boxHeight: 7,
          }
        },
        tooltip: {
          usePointStyle: true,
          multiKeyBackground: "#00000000"
        }
      }
    }
  });








  // System chart
  const system_ctx = document.getElementById('system_chart');
  let system_chart = new Chart(system_ctx, {
    type: 'line',
    data: {
      labels: [],
      datasets: [{
        label: 'Memory usage (%)',
        data: [],
        borderWidth: 3,
        borderColor: "#559598",
        backgroundColor: "#55959840",
        fill: true,
      }, {
        label: 'CPU temp (°C)',
        data: [],
        borderWidth: 3,
        borderColor: "#cc1d46",
        backgroundColor: "#cc1d4640",
        fill: true,
      }, {
        label: 'Disk usage (%)',
        data: [],
        borderWidth: 3,
        borderColor: "#d79921",
        backgroundColor: "#d7992120",
        fill: true,
      }],
    },
    options: {
      interaction: {
        intersect: false,
        mode: 'index',
      },
      scales: {
        y: {
          min: 0,
          max: 100,
        },
        x: {
          ticks: {
            maxTicksLimit: 2
          }
        }
      },
      animation: {
        duration: 0
      },
      maintainAspectRatio: false,
      cubicInterpolationMode: 'monotone',
      tension: 0.5,
      pointRadius: 0,
      pointHitRadius: 20,
      plugins: {
        legend: {
          labels: {
            usePointStyle: true,
            boxHeight: 7,
          }
        },
        tooltip: {
          usePointStyle: true,
          multiKeyBackground: "#00000000"
        }
      }
    }
  });











  const dbDataRequest = new Request("/api/v1/power_consumption?from=13.06.2024&to=13.06.2024");
  fetch(dbDataRequest).then((response) => {
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return response.json()
  }).then((dbDataObject) => {
    let entries = dbDataObject["data"];
    chart.data.labels = [];
    system_chart.data.datasets[0].data = [];
    system_chart.data.datasets[1].data = [];
    entries.forEach(element => {
      chart.data.labels.push(new Date(element["timestamp"]).toLocaleString("DE"));
      system_chart.data.datasets[0].data.push(element["power"]);        // Power used
      system_chart.data.datasets[1].data.push(element["voltage"]);      // PV power (to be replaced)
    });
    chart.update();
  });






  const systemDataRequest = new Request("/api/v1/system?from=13.06.2024&to=13.06.2024");
  fetch(systemDataRequest).then((response) => {
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return response.json()
  }).then((dbDataObject) => {
    let entries = dbDataObject["data"];
    chart.data.labels = [];
    system_chart.data.datasets[0].data = [];
    system_chart.data.datasets[1].data = [];
    system_chart.data.datasets[2].data = [];
    entries.forEach(element => {
      chart.data.labels.push(new Date(element["timestamp"]).toLocaleString("DE"));
      system_chart.data.datasets[0].data.push(element["used_memory_percent"]);
      system_chart.data.datasets[1].data.push(element["cpu_temperature"]);
      system_chart.data.datasets[2].data.push(element["used_disk_percent"]);
    });
    system_chart.update();
  });
}, false);
