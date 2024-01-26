// const people = [
//   {
//     name: 'Leslie Alexander',
//     email: 'leslie.alexander@example.com',
//     role: 'Co-Founder / CEO',
//     imageUrl:
//       'https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
//     href: '#',
//     lastSeen: '3h ago',
//     lastSeenDateTime: '2023-01-23T13:23Z',
//   },
//   {
//     name: 'Michael Foster',
//     email: 'michael.foster@example.com',
//     role: 'Co-Founder / CTO',
//     imageUrl:
//       'https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
//     href: '#',
//     lastSeen: '3h ago',
//     lastSeenDateTime: '2023-01-23T13:23Z',
//   },
//   {
//     name: 'Dries Vincent',
//     email: 'dries.vincent@example.com',
//     role: 'Business Relations',
//     imageUrl:
//       'https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
//     href: '#',
//     lastSeen: null,
//   },
//   {
//     name: 'Lindsay Walton',
//     email: 'lindsay.walton@example.com',
//     role: 'Front-end Developer',
//     imageUrl:
//       'https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
//     href: '#',
//     lastSeen: '3h ago',
//     lastSeenDateTime: '2023-01-23T13:23Z',
//   },
//   {
//     name: 'Courtney Henry',
//     email: 'courtney.henry@example.com',
//     role: 'Designer',
//     imageUrl:
//       'https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
//     href: '#',
//     lastSeen: '3h ago',
//     lastSeenDateTime: '2023-01-23T13:23Z',
//   },
//   {
//     name: 'Tom Cook',
//     email: 'tom.cook@example.com',
//     role: 'Director of Product',
//     imageUrl:
//       'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
//     href: '#',
//     lastSeen: null,
//   },
// ]

export function ResultList({ results }: Props) {
  return (
    <ul role="list" className="divide-y divide-gray-100">
      {results.map((result) => (
        <li
          key={result.key}
          className="relative flex justify-between gap-x-6 py-5"
        >
          <div className="flex min-w-0 gap-x-4">
            <div className="min-w-0 flex-auto">
              <p className="text-sm font-semibold leading-6 text-gray-900">
                <a href={result.url} target="_blank" className="flex items-center gap-2">
                  {result.title} <span className="text-xs text-gray-500 truncate block max-w-64">{result.url}</span>
                </a>
              </p>
              <p className="line-clamp-3 text-xs text-gray-700 bg-gray-100 rounded-md p-1">
                {result.text}
              </p>
            </div>
          </div>
          <div className="flex shrink-0 items-center gap-x-4">
            <div className="hidden sm:flex sm:flex-col sm:items-end">
              <p className="text-sm leading-6 text-gray-900">
                {result.distance.toLocaleString(undefined, {
                  maximumFractionDigits: 1,
                })}
              </p>
              {/* <p className="mt-1 text-xs leading-5 text-gray-500">
                <time dateTime={result.lastSeenDateTime}>
                  {result.lastSeen}
                </time>
              </p> */}
            </div>
          </div>
        </li>
      ))}
    </ul>
  );
}

interface Props {
  results: {
    key: string;
    title: string;
    text: string;
    url: string;
    distance: number;
  }[];
}