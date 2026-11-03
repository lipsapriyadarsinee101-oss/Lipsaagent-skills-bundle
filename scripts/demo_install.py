"""Record an actual, isolated install/reinstall/uninstall demonstration."""
import argparse
from pathlib import Path
import subprocess
import tempfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary', required=True)
parser.add_argument('--output', default='demo-transcript.txt')
args = parser.parse_args()
binary = Path(args.binary).resolve(strict=True)
lines = ['Skills Bundle: recorded command-line demonstration',
         'Targets: Claude Code, Codex and shared Agent Skills project folders.',
         'This checks file installation, not execution inside each agent application.', '']
with tempfile.TemporaryDirectory(prefix='skills-bundle-demo-') as temporary:
    root = Path(temporary)
    commands = [
        ['install', '--project', '--agent', 'all', '--copy', '--only', 'humanizer'],
        ['install', '--project', '--agent', 'all', '--copy', '--only', 'humanizer'],
        ['uninstall', '--project', '--agent', 'all', '--only', 'humanizer'],
    ]
    for index, command in enumerate(commands):
        result = subprocess.run([str(binary), *command], cwd=root,
                                capture_output=True, encoding='utf-8', errors='replace')
        lines += ['$ skills-bundle ' + ' '.join(command),
                  result.stdout.replace(str(root), '<demo-project>'),
                  result.stderr.replace(str(root), '<demo-project>'),
                  'Exit code: ' + str(result.returncode)]
        if result.returncode:
            raise RuntimeError('\n'.join(lines))
        for folder in ['.claude', '.codex', '.agents']:
            skill = root / folder / 'skills' / 'humanizer'
            assert skill.joinpath('SKILL.md').is_file() if index < 2 else not skill.exists()
        lines += ['Verified: all three targets ' + ('contain the skill.' if index < 2 else 'removed the installed skill.'), '']
lines.append('PASS: install, repeat install and uninstall across all three target folders.')
output = Path(args.output)
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text('\n'.join(lines), encoding='utf-8')
print('\n'.join(lines))
