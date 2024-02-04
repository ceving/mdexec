# IBM PC Floppy Size

| Name              | 360k | 720k | 1440k |
|-------------------|------|------|-------|
| Size (inch)       | 5¼   | 3½   | 3½    |
| Sides             | 2    | 2    | 2     |
| Tracks per Side   | 40   | 80   | 80    |
| Sectors per Track | 9    | 9    | 18    |
| Bytes per Sector  | 512  | 512  | 512   |

The following script creates three empty floppy images.

```bash
dd if=/dev/zero of=360K  bs=512 count=$((2*40*9))
dd if=/dev/zero of=720K  bs=512 count=$((2*80*9))
dd if=/dev/zero of=1440K bs=512 count=$((2*80*18))
cat > mtools.conf <<EOF
drive a: file="360K"
drive b: file="720K"
drive c: file="1440K"
EOF
export MTOOLSRC=mtools.conf
mformat a:
mformat b:
mformat c:
```

## Requirements

| [Mtools][1]    | `apt install mtools`    |
| [Coreutils][2] | `apt install coreutils` |
| [Bash][3]      | `apt install bash`      |

[1]: https://www.gnu.org/software/mtools/
[2]: https://www.gnu.org/software/coreutils/
[3]: https://www.gnu.org/software/bash/
