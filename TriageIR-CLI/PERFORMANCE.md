# TriageIR CLI Performance Guide

## Performance Characteristics

### Typical Collection Times

| Artifact Type | Typical Time | Notes |
|---------------|--------------|-------|
| System Info | < 1 second | Fast, minimal system calls |
| Processes (with hashes) | 5-30 seconds | Depends on number of processes and file sizes |
| Processes (no hashes) | 1-5 seconds | Much faster without hash calculation |
| Network Connections | 1-3 seconds | Fast, direct API calls |
| Persistence Mechanisms | 2-10 seconds | Registry and file system access |
| Event Logs | 10-60 seconds | Depends on log size and max_events setting |

### Memory Usage

- **Base memory**: ~10-20 MB
- **Process collection**: +1-5 MB (depends on process count)
- **Event logs**: +5-50 MB (depends on max_events setting)
- **Network connections**: +1-10 MB (depends on connection count)

### Disk Space Requirements

| Collection Type | Typical Output Size |
|----------------|-------------------|
| Minimal (system + processes, no hashes) | 100 KB - 1 MB |
| Standard (all artifacts, limited events) | 1-10 MB |
| Comprehensive (all artifacts, many events) | 10-100 MB |
| Full forensic (max events, all hashes) | 50-500 MB |

## Performance Optimization

### Fast Collection Options

```cmd
# Fastest possible collection (< 10 seconds)
triageir-cli.exe --only system,processes --skip-hashes --output fast.json

# Quick threat assessment (< 30 seconds)
triageir-cli.exe --skip-hashes --skip-events --output quick_threat.json

# Network-focused collection (< 15 seconds)
triageir-cli.exe --only processes,network,persistence --skip-hashes --output network_focus.json
```

### Balanced Performance Options

```cmd
# Good balance of speed and completeness (< 60 seconds)
triageir-cli.exe --max-events 500 --output balanced.json

# Skip only the slowest operations
triageir-cli.exe --skip-hashes --max-events 1000 --output semi_fast.json
```

### Comprehensive Collection

```cmd
# Full forensic collection (may take several minutes)
triageir-cli.exe --max-events 10000 --verbose --output comprehensive.json

# Maximum detail collection
triageir-cli.exe --max-events 50000 --verbose --output full_detail.json
```

## Performance Tuning by Use Case

### Incident Response (Speed Critical)

**Priority**: Get actionable intelligence quickly

```cmd
# Phase 1: Immediate threat assessment (< 30 seconds)
triageir-cli.exe --only processes,network --skip-hashes --output ir_phase1.json

# Phase 2: Persistence check (< 60 seconds total)
triageir-cli.exe --only persistence --output ir_phase2.json

# Phase 3: Full collection if time permits
triageir-cli.exe --skip-hashes --max-events 1000 --output ir_full.json
```

### Forensic Analysis (Completeness Critical)

**Priority**: Collect all available evidence

```cmd
# Complete evidence collection
triageir-cli.exe --verbose --max-events 10000 --output forensic_complete.json

# If time is unlimited
triageir-cli.exe --verbose --max-events 100000 --output forensic_exhaustive.json
```

### Automated Deployment (Resource Conscious)

**Priority**: Minimize system impact

```cmd
# Low-impact collection for mass deployment
triageir-cli.exe --skip-hashes --max-events 100 --output automated.json

# Staggered collection to reduce load
triageir-cli.exe --only system,processes --skip-hashes --output auto_phase1.json
# (run later)
triageir-cli.exe --only network,persistence --output auto_phase2.json
```

### Continuous Monitoring (Efficiency Critical)

**Priority**: Regular collection with minimal overhead

```cmd
# Hourly process monitoring
triageir-cli.exe --only processes --skip-hashes --output monitor_processes.json

# Daily comprehensive check
triageir-cli.exe --skip-hashes --max-events 500 --output daily_check.json

# Weekly full collection
triageir-cli.exe --max-events 5000 --output weekly_full.json
```

## System Resource Impact

### CPU Usage

- **Process enumeration**: Low-medium CPU usage
- **Hash calculation**: High CPU usage (can be skipped)
- **Event log parsing**: Medium CPU usage
- **Network enumeration**: Low CPU usage

### Disk I/O

- **Process hash calculation**: High disk I/O (reading executables)
- **Event log access**: Medium disk I/O
- **Registry access**: Low-medium disk I/O
- **Output writing**: Low disk I/O

### Memory Usage Patterns

- **Linear growth** with number of processes and connections
- **Significant increase** with event log collection
- **Minimal impact** from hash calculation (CPU-bound, not memory-bound)

## Performance Monitoring

### Built-in Timing

The tool automatically tracks and reports:
- Total scan duration in milliseconds
- Individual module completion times (in verbose mode)
- Artifact counts for performance correlation

### External Monitoring

```cmd
# Monitor resource usage during collection
wmic process where name="triageir-cli.exe" get ProcessId,PageFileUsage,WorkingSetSize /format:table

# Time the collection
powershell "Measure-Command { .\triageir-cli.exe --output test.json }"
```

## Troubleshooting Performance Issues

### Slow Process Collection

**Symptoms**: Process enumeration takes > 60 seconds
**Solutions**:
- Use `--skip-hashes` to avoid file I/O
- Check for processes with large executables
- Verify disk performance

### High Memory Usage

**Symptoms**: Tool uses > 500 MB RAM
**Solutions**:
- Reduce `--max-events` setting
- Use `--only` to limit artifact types
- Check for systems with excessive processes/connections

### Large Output Files

**Symptoms**: Output files > 100 MB
**Solutions**:
- Use `--max-events` to limit event log entries
- Use `--only` to collect specific artifacts
- Consider splitting collection into multiple runs

### Slow Event Log Collection

**Symptoms**: Event log collection takes > 5 minutes
**Solutions**:
- Reduce `--max-events` setting
- Use `--skip-events` for faster collection
- Check Windows Event Log service status

## Benchmarking

### Test System Specifications

When benchmarking, document:
- CPU model and speed
- RAM amount and speed
- Disk type (SSD vs HDD)
- Windows version and build
- Number of running processes
- Event log sizes

### Benchmark Commands

```cmd
# Baseline performance test
triageir-cli.exe --verbose --output benchmark_full.json

# Speed test
triageir-cli.exe --skip-hashes --skip-events --verbose --output benchmark_fast.json

# Memory test
triageir-cli.exe --max-events 10000 --verbose --output benchmark_memory.json
```

### Performance Regression Testing

```cmd
# Standard performance test suite
triageir-cli.exe --only system --output perf_system.json
triageir-cli.exe --only processes --skip-hashes --output perf_processes_nohash.json
triageir-cli.exe --only processes --output perf_processes_hash.json
triageir-cli.exe --only network --output perf_network.json
triageir-cli.exe --only persistence --output perf_persistence.json
triageir-cli.exe --only events --max-events 100 --output perf_events.json
```

## Best Practices for Performance

1. **Profile your environment** - Test different options to find optimal settings
2. **Use appropriate options** - Don't collect more than you need
3. **Consider system load** - Avoid running during peak usage
4. **Monitor resource usage** - Watch for memory and CPU spikes
5. **Test before deployment** - Validate performance in your environment
6. **Document performance** - Keep records of typical collection times