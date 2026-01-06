import { Injectable } from '@angular/core';
import {environment} from '../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {LoginService} from '../login.service';

export interface DataObjectsPageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: DataObject[];
}
export interface DataObject {
  id: number;
  name: string;
}

@Injectable({
  providedIn: 'root',
})
export class DataObjectsService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) {}

  getDataObjects():Observable<DataObjectsPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<DataObjectsPageInfo>(`${this.apiUrl}/api/data_objects`, {headers});
  }


}
