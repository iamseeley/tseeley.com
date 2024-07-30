---
title: enter react 
date: 2024-07-29
---

still working away on [web analytics](/logs/web-analytics). 

i decided to port the frontend to react.

i also used [globe.gl](https://globe.gl/) to render a globe displaying visitor origins!

![globe pic](/images/globepic.png)

some key use cases that made react well-suited for the project:

**managing connection status and error states**: 
useState manages the connectionStatus and error states, so there is real-time feedback on the server-sent events (SSE) connection's health.

**real-time updates for analytics data**:
useState and useEffect hooks manage the SSE connection. so, the dashboard can recieve and display live updates from the analytics stream.

```javascript
const setupDataStream = useCallback(() => {
    if (eventSourceRef.current) {
      eventSourceRef.current.close();
    }

    const eventSource = new EventSource(STREAM_URL);
    eventSourceRef.current = eventSource;

    eventSource.onopen = () => {
      console.log('SSE connection opened');
      setConnectionStatus('Connected');
    };

    eventSource.onmessage = (event) => {
      try {
        const newData = JSON.parse(event.data);
        setAnalyticsData(prevData => {
          if (JSON.stringify(prevData) !== JSON.stringify(newData)) {
            return newData;
          }
          return prevData;
        });
        setIsLoading(false);
      } catch (error) {
        console.error('Error parsing SSE data:', error);
        setError('Failed to parse analytics data.');
      }
    };

    eventSource.onerror = (error) => {
      console.error('SSE Error:', error);
      setError('Connection error. Will attempt to reconnect.');
      setConnectionStatus('Error - Reconnecting');
    };

    return eventSource;
  }, []);

  useEffect(() => {
    const eventSource = setupDataStream();
    return () => {
      if (eventSource) {
        eventSource.close();
      }
    };
  }, [setupDataStream]);
```

**organizing dashboard into components**:
the home page and site page dashboards contain a lot of code that can be shared between them. by creating a few reusable dashboard components it made the code much more manageable. 


 **chart rendering**:
the useMemo hook optimizes the calculation of chart data, preventing unnecessary recalculations when other parts of the state change.

```javascript
const chartData = useMemo(() => ({
    browsers: aggregateData('browsers'),
    os: aggregateData('os'),
    screenSizes: aggregateData('screenSizes'),
    timeData: aggregateTimeData(),
    countries: aggregateData('countries')
  }), [analyticsData, aggregateData, aggregateTimeData]);
```

 **aggregating data for top paths and referrers**: 
useCallback optimizes functions like aggregateData and aggregateReferrers, making sure they're not recreated on every render.

```javascript
const aggregateData = useCallback((key) => {
    if (!analyticsData) return {};
    const aggregatedData = {};
    Object.values(analyticsData).forEach(site => {
      Object.values(site.paths || {}).forEach(path => {
        Object.entries(path[key] || {}).forEach(([item, count]) => {
          let mappedItem;
          if (key === 'browsers') {
            mappedItem = mapName(item, browserMap);
          } else if (key === 'os') {
            mappedItem = mapName(item, osMap);
          } else if (key === 'screenSizes') {
            mappedItem = categorizeScreenSize(parseInt(item.split('x')[0]));
          } else {
            mappedItem = item;
          }
          aggregatedData[mappedItem] = (aggregatedData[mappedItem] || 0) + count;
        });
      });
    });
    return aggregatedData;
  }, [analyticsData]);
```



