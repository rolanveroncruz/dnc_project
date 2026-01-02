import {DestroyRef, Component, OnInit, signal, inject} from '@angular/core';
import {DentalServicesService, DentalServicesPageInfo } from '../../../api_services/dental-services-service';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatDialog} from '@angular/material/dialog';
import {AddEditDentalServices} from './add-edit-dental-services/add-edit-dental-services';
import {MatCard, MatCardContent, MatCardHeader} from '@angular/material/card';

type LoadState = 'loading' | 'loaded' | 'error';

export interface DentalService {
  id: number;
  name: string;
  type_name: string;
  record_tooth: boolean;
  active: boolean;
  last_modified_by: string;
  last_modified_on: Date;
}
@Component({
  selector: 'app-setup-dental-services',
  imports: [
    GenericDataTableComponent,
    MatCard,
    MatCardHeader,
    MatCardContent,
  ],
  templateUrl: './dental-services.html',
  styleUrl: './dental-services.scss',
})
export class DentalServices implements OnInit{
  state = signal<LoadState>('loading');
  dentalServices = signal<DentalService[] |null>(null);
  errorMsg = signal<string | null>(null);
  private destroyRef = inject(DestroyRef);
  private dialog = inject(MatDialog);
  dentalServicesColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'type_name', label: 'Type'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];

  openRowDialog(row:any){
    const ref = this.dialog.open(AddEditDentalServices, {
      data:row,
      width: '720px',
      maxWidth: '95vw',
      disableClose : false,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }

  constructor(
    private dentalServicesService: DentalServicesService,
  ){}


   ngOnInit(): void {
     this.load();

   }
   private load(){
      this.state.set('loading');
      this.errorMsg.set(null)
     this.dentalServicesService.getDentalServices()
       .pipe(takeUntilDestroyed(this.destroyRef))
       .subscribe({
         next: (res) => {
           this.dentalServices.set(res.items);
           this.state.set('loaded');
         },
         error: (err) => {
           console.log(err);
           this.errorMsg.set("Failed to load dental services");
           this.state.set('error');
         }
       })
   }
}
