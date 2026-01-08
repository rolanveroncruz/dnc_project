import {DestroyRef, Component, OnInit, signal, inject} from '@angular/core';
import {DentalServicesService, DentalServicesPageInfo } from '../../../api_services/dental-services-service';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {MatDialog} from '@angular/material/dialog';
import {DentalServiceDialogComponent} from './add-edit-dental-services/add-edit-dental-services';
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';
import {DentalServiceType, DentalServiceTypesService} from '../../../api_services/dental-service-types-service';
import {DentalServiceDialogData} from './add-edit-dental-services/dental-service-dialog.models';

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
    MatCardSubtitle,
    MatCardTitle,
  ],
  templateUrl: './dental-services.html',
  styleUrl: './dental-services.scss',
})
export class DentalServices implements OnInit{
  state = signal<LoadState>('loading');
  dentalServices = signal<DentalService[] |null>(null);
  errorMsg = signal<string | null>(null);
  dentalServiceTypes = signal<DentalServiceType[]>([]);


  private destroyRef = inject(DestroyRef);
  private dialog = inject(MatDialog);
  dentalServicesColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'type_name', label: 'Type'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];

  openEditDialog(row:any){
    const data: DentalServiceDialogData={
      mode: 'edit',
      service: row,
      typeOptions: this.dentalServiceTypes(),
      currentUserName: 'test',
    };
    const ref = this.dialog.open(DentalServiceDialogComponent, {
      data,
      disableClose : true,
      autoFocus: false,
      width: '720px',
      maxWidth: '92vw',
    });

    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }

  constructor(
    private dentalServicesService: DentalServicesService,
    private dentalServiceTypesService: DentalServiceTypesService,
  ){}


   ngOnInit(): void {
     this.load_dental_services();
     this.load_dental_service_types();

   }
   private load_dental_services(){
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
   private load_dental_service_types(){
    this.dentalServiceTypesService.getDentalServiceTypes()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.dentalServiceTypes.set(res.items);
        },
        error: (err) => {
          console.log("Failed to load dental service types",err);
        }

      })
   }
}
