import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BasicServicesTabComponent } from './basic-services-tab-component';

describe('BasicServicesTabComponent', () => {
  let component: BasicServicesTabComponent;
  let fixture: ComponentFixture<BasicServicesTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BasicServicesTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BasicServicesTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
