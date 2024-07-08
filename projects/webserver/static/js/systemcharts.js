// import {de} from './chartjs-adapter-date-fns.bundle.min.js/locale';

const timezoneOffset = new Date().getTimezoneOffset();
let from = new Date();
from.setDate(from.getDate() - 5);
from.setHours(0);
let to = new Date();

Date.prototype.correctUTC = function(timezoneOffset) {
  this.setTime(this.getTime() - (timezoneOffset*60*1000));
  return this;
}

let chartOptions = {
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
        includeBounds: true,
        autoSkip: true,
        padding: 10,
        maxRotation: 0,
        minRotation: 0,
      },
      type: 'time',
      time: {
        unit: 'minute',
        displayFormats: {
          day: 'yy.MM.dd hh:mm'
        }
      }
    }
  },
  animation: {
    duration: 500
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
};

// Clone first
let powerChartOptions = JSON.parse(JSON.stringify(chartOptions));

// Adaptions
powerChartOptions.scales.y.max = 12;


document.addEventListener('DOMContentLoaded', function() {
  const power_ctx = document.getElementById('power_chart');
  let power_chart = new Chart(power_ctx, {
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
    options: powerChartOptions,
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
    options: chartOptions
  });










  function updatePowerChart() {
    const dbDataRequest = new Request("/api/v1/power_consumption?from="+String(Number(from))+"&to="+String(Number(to)));
    fetch(dbDataRequest).then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return response.json()
    }).then((dbDataObject) => {
      let entries = dbDataObject["data"];
      power_chart.data.labels = [];
      power_chart.data.datasets[0].data = [];
      power_chart.data.datasets[1].data = [];
      entries.forEach(element => {
        power_chart.data.labels.push(new Date(element["timestamp"]).correctUTC(timezoneOffset));
        power_chart.data.datasets[0].data.push(element["power"]);        // Power used
        power_chart.data.datasets[1].data.push(element["voltage"]);      // PV power (to be replaced)
      });
      power_chart.update();
    });
  }




  function updateSystemChart() {
    const systemDataRequest = new Request("/api/v1/system?from="+String(Number(from))+"&to="+String(Number(to)));
    fetch(systemDataRequest).then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return response.json()
    }).then((dbDataObject) => {
      let entries = dbDataObject["data"];
      system_chart.data.labels = [];
      system_chart.data.datasets[0].data = [];
      system_chart.data.datasets[1].data = [];
      system_chart.data.datasets[2].data = [];
      entries.forEach(element => {
        system_chart.data.labels.push(new Date(element["timestamp"]).correctUTC(timezoneOffset));
        system_chart.data.datasets[0].data.push(element["used_memory_percent"]);
        system_chart.data.datasets[1].data.push(element["cpu_temperature"]);
        system_chart.data.datasets[2].data.push(element["used_disk_percent"]);
      });
      system_chart.update();
    });
  }

  updatePowerChart();
  updateSystemChart();
  const updatePowerChartInterval = setInterval(updatePowerChart, 5000);
  const updateSystemChartInterval = setInterval(updateSystemChart, 5000);
}, false);
